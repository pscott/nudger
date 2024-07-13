use axum::Router;
use std::env;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
extern crate dotenv;
use ::axum::extract::Json;
use ::axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Extension;
use dotenv::dotenv;
use ethers::types::Address;
use nudger::filters;
use nudger::nudge::Nudge;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum ServerError {
    ErrorString(String),
}

impl<T: std::string::ToString + Sized> From<T> for ServerError {
    fn from(err: T) -> Self {
        ServerError::ErrorString(err.to_string())
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ErrorString(body) => {
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    let _ = tracing::subscriber::set_global_default(subscriber);
    tracing::info!("Starting server...");

    let port: u16 = env::var("PORT")
        .map(|val| val.parse::<u16>().unwrap())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app()).await.unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateNudgeParams {
    protocol: String,
    cta: String,
    filter_name: String,
}

async fn handle_create_nudge(
    Extension(state): Extension<State>,
    Json(p): Json<Value>,
) -> Result<impl IntoResponse, ServerError> {
    let request: CreateNudgeParams = serde_json::from_value(p)?;
    let mut nudges = state.nudges.lock().await;

    let nudge = Nudge {
        protocol: request.protocol.clone(),
        cta: request.cta.clone(),
        filter_name: request.filter_name.to_string(),
    };
    tracing::info!(nudge = ?nudge);
    nudges.push(nudge);

    Ok("OK") // wtf?
}

#[derive(Debug, Deserialize)]
struct GetNudgeParams {
    target: Address,
}

#[derive(Debug, Clone, Serialize)]
struct GetNudgeResponse {
    protocol: String,
    text: String,
    cta: String,
}

async fn handle_get_nudge(
    Extension(state): Extension<State>,
    Json(p): Json<Value>,
) -> Result<impl IntoResponse, ServerError> {
    let params: GetNudgeParams = serde_json::from_value(p)?;
    let target = params.target;

    // Finds the corresponding nudge. Doesn't handle the case where the user is eligible to multiple nudges.
    let nudges = state.nudges.lock().await;

    // Collect futures for all filter checks
    let filter_futures: Vec<_> = nudges
        .iter()
        .map(|nudge| {
            let state = state.clone(); // todo maybe unecessary
            async move {
                if let Some(filter) = state.filters.get(&nudge.filter_name) {
                    if let Some(text) = filter(state.client.clone(), &target).await {
                        return Some(GetNudgeResponse {
                            protocol: nudge.protocol.clone(),
                            text,
                            cta: nudge.cta.clone(),
                        });
                    }
                }
                None
            }
        })
        .collect();

    // Execute all futures concurrently and find the first non-None response
    let responses = futures::future::join_all(filter_futures).await;
    let response = responses.into_iter().find_map(|res| res);

    match response {
        Some(nudge) => Ok(Json(nudge.clone())),
        None => Err(ServerError::ErrorString("No nudge found".to_string())),
    }
}

pub async fn handle_health() -> Result<impl IntoResponse, ()> {
    Ok(axum::response::Html("Healthy!"))
}

type FilterFn =
    fn(reqwest::Client, &Address) -> Pin<Box<dyn Future<Output = Option<String>> + Send + '_>>;

#[derive(Clone)]
pub struct State {
    pub client: reqwest::Client,
    pub nudges: Arc<Mutex<Vec<Nudge>>>, // TODO: This should not be stored in memory and definitely NOT be stored as a vec
    pub filters: HashMap<String, FilterFn>,
}

// TODO this is temporarily needed
fn aave_resolve_wrapper(
    client: reqwest::Client,
    target: &Address,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send + '_>> {
    Box::pin(filters::aave::resolve(client, target))
}

fn create_filters() -> HashMap<String, FilterFn> {
    let mut filters: HashMap<String, FilterFn> = HashMap::new();
    filters.insert("aave".to_string(), aave_resolve_wrapper as FilterFn);

    filters
}

fn app() -> Router {
    dotenv().ok();

    let client = reqwest::Client::new();
    let nudges = Arc::new(Mutex::new(vec![])); // Not the correct structure

    let filters = create_filters();

    let state = State {
        client,
        nudges,
        filters,
    };

    Router::new()
        .route("/create-nudge", post(handle_create_nudge))
        .route("/get-nudge", post(handle_get_nudge)) // should be a get request
        .route("/health", get(handle_health))
        .layer(Extension(state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_create_and_get_nudge() {
        // Assuming you have a function to create your application

        // Create a test client
        let client = TestServer::new(app()).unwrap();

        // Create a nudge
        let _ = client
            .post("/create-nudge")
            .json(&json!({
                "protocol": "Aave",
                "cta": "Click here",
                "filter_name": "aave",
            }))
            .await;

        // Retrieve the nudge
        let get_response = client // TODO: This should be a get request
            .post("/get-nudge")
            .json(&json!({"target": "0x3e8734ec146c981e3ed1f6b582d447dde701d90c"}))
            .await;

        get_response.assert_text_contains("Aave");
        get_response.assert_text_contains("amazing CTA");
    }
}
