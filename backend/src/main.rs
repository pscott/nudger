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
use nudger::filters;
use nudger::nudge::Nudge;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};

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
        .unwrap_or(80);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app()).await.unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateNudgeParams {
    cta_url: String,
    cta_text: String,
    filter_name: String,
}

async fn handle_create_nudge(
    Extension(state): Extension<State>,
    Json(p): Json<Value>,
) -> Result<impl IntoResponse, ServerError> {
    let request: CreateNudgeParams = serde_json::from_value(p)?;
    let mut nudges = state.nudges.lock().await;

    let nudge = Nudge {
        cta_url: request.cta_url.clone(),
        cta_text: request.cta_text.clone(),
        filter_name: request.filter_name.to_string(),
    };
    tracing::info!(nudge = ?nudge);
    nudges.push(nudge);

    Ok("OK") // wtf?
}

#[derive(Debug, Deserialize)]
struct GetNudgeParams {
    // TMP: this should be an "address"
    target: String,
}

#[derive(Debug, Clone, Serialize)]
struct GetNudgeResponse {
    text: String,
    cta_url: String,
    cta_text: String,
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
            let target = target.clone();
            async move {
                tracing::info!("Checking nudge: {:?}", nudge);
                if let Some(filter) = state.filters.get(&nudge.filter_name) {
                    tracing::info!("in let some 1");
                    if let Some(text) = filter(state.client.clone(), target).await {
                        tracing::info!("in let some 2");
                        return Some(GetNudgeResponse {
                            text,
                            cta_url: nudge.cta_url.clone(),
                            cta_text: nudge.cta_text.clone(),
                        });
                    }
                }
                None
            }
        })
        .collect();

    // Execute all futures concurrently and find the first non-None response
    let responses = futures::future::join_all(filter_futures).await;
    let response = responses.into_iter().find_map(|res| res); // wtf?
    tracing::info!("response: {:?}", response);

    match response {
        Some(nudge) => Ok(Json(nudge.clone())),
        None => Ok(Json(GetNudgeResponse {
            // Default message is Lido nudge. No reason, this is just a placeholder
            text: "Don't let your ETH stay idle.".to_string(),
            cta_url: "https://stake.lido.finance/".to_string(),
            cta_text: "Start staking!".to_string(),
        })),
    }
}

pub async fn handle_health() -> Result<impl IntoResponse, ()> {
    Ok(axum::response::Html("Healthy!"))
}

type FilterFn = fn(reqwest::Client, String) -> Pin<Box<dyn Future<Output = Option<String>> + Send>>;

#[derive(Clone)]
pub struct State {
    pub client: reqwest::Client,
    pub nudges: Arc<Mutex<Vec<Nudge>>>, // TODO: This should not be stored in memory and definitely NOT be stored as a vec
    pub filters: HashMap<String, FilterFn>,
}

// TODO this is temporarily needed
fn aave_resolve_wrapper(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(filters::aave::resolve(client, target))
}

fn zksync_resolve_wrapper(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(filters::zksync::resolve(client, target))
}

fn memes_resolve_wrapper(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(filters::memes::resolve(client, target))
}

fn collector_resolve_wrapper(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(filters::collector::resolve(client, target))
}

fn create_filters() -> HashMap<String, FilterFn> {
    let mut filters: HashMap<String, FilterFn> = HashMap::new();
    filters.insert("aave".to_string(), aave_resolve_wrapper as FilterFn);
    filters.insert("zksync".to_string(), zksync_resolve_wrapper as FilterFn);
    filters.insert("memes".to_string(), memes_resolve_wrapper as FilterFn);
    filters.insert(
        "collector".to_string(),
        collector_resolve_wrapper as FilterFn,
    );

    filters
}

#[derive(Debug, Serialize)]
struct InfoResponse {
    name: String,
    version: String,
}

async fn handle_root(
    Extension(_state): Extension<State>,
) -> Result<impl IntoResponse, ServerError> {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    Ok(Json(InfoResponse {
        version: version.to_string(),
        name: name.to_string(),
    }))
}

// This is a temporary function to add some nudges to the nudges vec, since the frontend won't call this function.
fn create_nudges() -> Vec<Nudge> {
    let nudges = vec![
        // Aave
        Nudge {
            cta_url: "https://app.aave.com/".to_string(),
            cta_text: "Just use Aave".to_string(),
            filter_name: "aave".to_string(),
        },
        // ZkSync
        Nudge {
            cta_url: "https://claim.zknation.io/".to_string(),
            cta_text: "Claim your $ZK airdrop!".to_string(),
            filter_name: "zksync".to_string(),
        },
        // Memes
        Nudge {
            cta_url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
            cta_text: "View our Degen leaderboard.".to_string(),
            filter_name: "memes".to_string(),
        },
        // Collector
        Nudge {
            cta_url: "https://opensea.io/".to_string(),
            cta_text: "Trade them on OpenSea".to_string(),
            filter_name: "collector".to_string(),
        },
    ];

    nudges
}

fn app() -> Router {
    dotenv().ok();

    let client = reqwest::Client::new();

    let filters = create_filters();

    // Create a CORS layer that allows all requests
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let nudge_vec = create_nudges();
    let nudges = Arc::new(Mutex::new(nudge_vec));

    let state = State {
        client,
        nudges,
        filters,
    };

    Router::new()
        .route("/create-nudge", post(handle_create_nudge)) // ideally this endpoint would exist but it's disabled and embedded directly for now
        .route("/get-nudge", post(handle_get_nudge)) // should be a get request
        .route("/health", get(handle_health))
        .route("/", get(handle_root))
        .layer(Extension(state))
        .layer(cors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_aave_nudge() {
        // construct a subscriber that prints formatted traces to stdout
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // use that subscriber to process traces emitted after this point
        let _ = tracing::subscriber::set_global_default(subscriber);
        tracing::info!("Starting server...");

        // Create a test client
        let client = TestServer::new(app()).unwrap();

        // Retrieve the nudge
        let get_response = client // TODO: This should be a get request
            .post("/get-nudge")
            .json(&json!({"target": "0x3e8734ec146c981e3ed1f6b582d447dde701d90c"}))
            .await;

        if get_response.status_code() != 200 {
            println!("Error getting nudge: {:?}", get_response.text());
        }

        get_response.assert_text_contains("Aave");
        get_response.assert_text_contains("You are missing out on");
    }

    #[tokio::test]
    async fn test_zksync_nudge() {
        // Create a test client
        let client = TestServer::new(app()).unwrap();

        // Retrieve the nudge
        let get_response = client // TODO: This should be a get request
            .post("/get-nudge")
            .json(&json!({"target": "0x59F3eb596b0af6ebB197362c5F191A9AbADd4F7b"}))
            .await;

        if get_response.status_code() != 200 {
            println!("Error getting nudge: {:?}", get_response.text());
        }

        get_response.assert_text_contains("Pudgy");
    }

    #[tokio::test]
    async fn test_memes_nudge() {
        // construct a subscriber that prints formatted traces to stdout
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // use that subscriber to process traces emitted after this point
        let _ = tracing::subscriber::set_global_default(subscriber);
        tracing::info!("Starting server...");
        // Create a test client
        let client = TestServer::new(app()).unwrap();

        // Retrieve the nudge
        let get_response = client
            .post("/get-nudge")
            .json(&json!({"target": "0x3f08f17973ab4124c73200135e2b675ab2d263d9"}))
            .await;

        if get_response.status_code() != 200 {
            println!("Error getting nudge: {:?}", get_response.text());
        }

        get_response.assert_text_contains("shitcoin");
    }

    #[tokio::test]
    async fn test_collector_nudge() {
        // construct a subscriber that prints formatted traces to stdout
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // use that subscriber to process traces emitted after this point
        let _ = tracing::subscriber::set_global_default(subscriber);
        tracing::info!("Starting server...");
        // Create a test client
        let client = TestServer::new(app()).unwrap();

        // Retrieve the nudge
        let get_response = client
            .post("/get-nudge")
            .json(&json!({"target": "0x5EF29cf961cf3Fc02551B9BdaDAa4418c446c5dd"}))
            .await;

        if get_response.status_code() != 200 {
            println!("Error getting nudge: {:?}", get_response.text());
        }

        get_response.assert_text_contains("NFT");
    }
}
