use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;

// Example of a nudge ZkSync might have wanted for their airdrop
// Wants to target anyone with a Pudgy Penguin

pub fn resolve(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(async move {
        let pudgy_collection_id = "0x59f3eb596b0af6ebb197362c5f191a9abadd4f7b:291688";

        let url = format!(
            "https://api.zerion.io/v1/wallets/{}/nft-collections/?filter[chain_ids]=ethereum&currency=usd",
            target
        );
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

        // This shoudl be re-written, not the cleanest way
        let mut found = false;
        for item in arr {
            let id = item.get("id")?.as_str()?;
            if id == pudgy_collection_id {
                tracing::info!("{:?}", item);
                // Maybe should check that `nft_counts > 0` but idk
                found = true;
                break;
            }
        }

        if !found {
            return None;
        }

        let text = "You have a Pudgy Penguin!".to_string();
        tracing::info!("text: {:?}", text);

        Some(text)
    })
}
