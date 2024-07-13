use axum::Router;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
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
    text: String,
    cta: String,
    filter_name: String,
}

async fn handle_create_nudge(
    Extension(state): Extension<State>,
    Json(p): Json<Value>,
) -> Result<impl IntoResponse, ServerError> {
    let request: CreateNudgeParams = serde_json::from_value(p)?;
    let mut nudges = state.nudges.lock().unwrap(); // should be async

    let nudge = Nudge {
        protocol: request.protocol.clone(),
        text: request.text.clone(),
        cta: request.cta.clone(),
        filter_name: request.filter_name.to_string(),
    };
    tracing::info!(nudge = ?nudge);
    nudges.push(nudge);

    Ok("OK")
}

#[derive(Debug, Deserialize)]
struct GetNudgeParams {
    target: Address,
}

async fn handle_get_nudge(
    Extension(state): Extension<State>,
    Json(p): Json<Value>,
) -> Result<impl IntoResponse, ServerError> {
    let params: GetNudgeParams = serde_json::from_value(p)?;
    let target = params.target;

    // Finds the corresponding nudge. Doesn't handle the case where the user is eligible to multiple nudges.
    let nudges = state.nudges.lock().unwrap(); // should be async

    let result = nudges
        .iter()
        .find(|nudge| match state.filters.get(&nudge.filter_name) {
            Some(filter) => filter(&target),
            None => false,
        });

    match result {
        Some(nudge) => Ok(Json(nudge.clone())),
        None => Err(ServerError::ErrorString("No nudge found".to_string())),
    }
}

pub async fn handle_health() -> Result<impl IntoResponse, ()> {
    Ok(axum::response::Html("Healthy!"))
}

type FilterFn = fn(&Address) -> bool;

#[derive(Clone)]
pub struct State {
    pub client: reqwest::Client,
    pub nudges: Arc<Mutex<Vec<Nudge>>>, // TODO: This should not be stored in memory and definitely NOT be stored as a vec
    pub filters: HashMap<String, FilterFn>,
}

fn app() -> Router {
    dotenv().ok();

    let client = reqwest::Client::new();
    let nudges = Arc::new(Mutex::new(vec![])); // Not the correct structure
    let mut filters: HashMap<String, FilterFn> = HashMap::new();
    filters.insert("aave".to_string(), filters::aave::is_eligible);
    // Create filters

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
                "text": "Hello, world!",
                "cta": "Click here",
                "filter_name": "aave",
            }))
            .await;

        // Retrieve the nudge
        let get_response = client // TODO: This should be a get request
            .post("/get-nudge")
            .json(&json!({"target": "0xd6fcfbe5d76d6ce0e77f00f5a370f8c677ea7150"}))
            .await;

        get_response.assert_text_contains("Aave");
    }
}
