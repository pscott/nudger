use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;

/// Example of a nudge Aave might want to create
/// Wants to target anyone with >10k USDT in their wallet

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
        let response = client
            .get(url)
            .header("authorization", zerion_api_key)
            .header("accept", "application/json")
            .send()
            .await
            .unwrap();

        // Parse the response JSON
        let response_json: Value = response.json().await.unwrap();

        // Iterate through the `data` array and check for the specific `id`
        //  let maybe_balance: Option<String> = if let Some(data) = response_json.get("data").and_then(|d| d.as_array()) {
        //      for item in data {
        //          if let Some(id) = item.get("id").and_then(|id| id.as_str()) {
        //              if id == usdt_mainnet_id {
        //                  if let Some(attributes) = item.get("attributes") {
        //                      if let Some(value) = attributes.get("value").and_then(|v| v.as_str()) {
        //                          Some(value.to_string());
        //                      }
        //                  }
        //              }
        //          }
        //      }
        //      None
        //  };

        let limit = 10_000.0;
        let balance = 1.0;
        // If balance > 10k, write "You are missing out on ${APY / 12 * balance} per month! Just use aave"
        // Check if the user is eligible for the nudge

        let text = format!(
            "You are missing out on ${} per month! Just use Aave",
            apy / balance * limit
        );

        Some(text)
    })
}
