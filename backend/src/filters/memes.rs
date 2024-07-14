use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;

// Example of a nudge for fun.
// Displays a message if you hold one one of $SHIB, $PEPE, $FLOKI
const SHIB_ID: &str = "0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce";
const PEPE_ID: &str = "0x6982508145454ce325ddbe47a25d4ec3d2311933";
const FLOKI_ID: &str = "0xcf0c122c6b73ff809c693db761e7baebe62b6a2e";

// request = https://api.zerion.io/v1/wallets/0x3f08f17973ab4124c73200135e2b675ab2d263d9/positions/?filter[positions]=only_simple&currency=usd&filter[chain_ids]=ethereum&filter[fungible_ids]=0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce&filter[trash]=only_non_trash&sort=value"

pub fn resolve(
    client: reqwest::Client,
    target: String,
) -> Pin<Box<dyn Future<Output = Option<String>> + Send>> {
    Box::pin(async move {
        let fungible_ids = format!("{},{},{}", SHIB_ID, PEPE_ID, FLOKI_ID);
        let url = format!("https://api.zerion.io/v1/wallets/{target:}/positions/?filter[positions]=only_simple&currency=usd&filter[chain_ids]=ethereum&filter[fungible_ids]={fungible_ids:}&filter[trash]=only_non_trash&sort=value");
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

        // Iterate through the `data` and sum the values
        let data: &Value = response_json.get("data")?;
        let arr = data.as_array()?;

        // Display the total value in shitcoin and suggest to buy ETH instead

        let mut total_balance = 0.0;
        for item in arr {
            let attributes = item.get("attributes")?;
            let balance = attributes.get("value")?.as_f64()?;
            total_balance += balance;
        }

        tracing::info!("total shitcoin balance: {:?}", total_balance);

        let text;
        if total_balance == 0.0 {
            return None;
        } else if total_balance < 10_000.0 {
            text = format!("You have ${total_balance:.0} worth of shitcoins.");
        } else if total_balance < 100_000.0 {
            text = format!("You have ${total_balance:.0} decent balance of shitcoins.");
        } else {
            text = format!("${total_balance:.0} worth of shitcoins. Such a degen.");
        }

        tracing::info!("text: {:?}", text);

        Some(text)
    })
}
