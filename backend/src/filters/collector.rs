use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;

// Example of a collector nudge (for fun).
// Displays a message if you have more than 3 NFTs on mainnet

pub fn resolve(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(async move {
        let url = format!(
            "https://api.zerion.io/v1/wallets/{}/nft-collections/?filter[chain_ids]=ethereum&currency=usd",
            target
        );
        tracing::info!("url: {:?}", url);
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
        let mut num = 0;
        for item in arr {
            let attr = item.get("attributes")?;
            let nft_count_str = attr.get("nfts_count")?.as_str()?;
            let nft_count = nft_count_str.parse::<u32>().unwrap();
            num += nft_count;
        }

        if num < 3 {
            return None;
        }

        let text = format!("You've collected {num:} NFTs on mainnet.");
        tracing::info!("text: {:?}", text);

        Some(text)
    })
}
