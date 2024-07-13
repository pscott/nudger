use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;

/// Example of a nudge Aave might want to create
/// Wants to target anyone with >10k USDT in their wallet
const MONTHS_IN_YEAR: f64 = 12.0;

pub fn resolve(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(async move {
        // Call Zerion and check USDT balance
        let apy = 0.1;
        let usdt_mainnet_id = "0xdac17f958d2ee523a2206206994597c13d831ec7-ethereum-asset-asset";

        let url = format!("https://api.zerion.io/v1/wallets/{}/positions/?filter[positions]=only_simple&currency=usd&filter[trash]=only_non_trash&sort=value", target);
        let zerion_api_key = env::var("ZERION_API_KEY").unwrap();
        let zerion_auth = format!("Basic {}", zerion_api_key);
        let response = client
            .get(url)
            .header("authorization", zerion_auth)
            .header("accept", "application/json")
            .send()
            .await
            .ok()?;

        // Parse the response JSON
        let response_json: Value = response.json().await.ok()?;

        // Iterate through the `data` array and check for the specific `id`
        let data: &Value = response_json.get("data")?;
        let arr = data.as_array()?;
        let mut balance: Option<f64> = None;

        // This shoudl be re-written, not the cleanest way
        for item in arr {
            let id = item.get("id")?.as_str()?;
            if id == usdt_mainnet_id {
                let attributes = item.get("attributes")?;
                balance = attributes.get("value")?.as_f64();
                break;
            }
        }

        let balance = balance?; // this is ugly

        tracing::info!("balance: {:?}", balance);

        let threshold = 10_000.0;

        if balance < threshold {
            return None;
        }

        let text = format!(
            "You are missing out on ${:.0} per month! Just use Aave",
            apy / MONTHS_IN_YEAR * balance
        );
        tracing::info!("text: {:?}", text);

        Some(text)
    })
}
