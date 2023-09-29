

pub mod order_service {
    use reqwest::Error;

    pub async fn validate_symbol(symbol: &str) -> Result<bool, Error> {
        let url = format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol);
        println!("a");
        // Send a Get request to the NASDAQ API
        //let response = reqwest::get(&url).await?;
        let response = reqwest::get(&url).await.map_err(|err| {
            println!("Error sending request: {:?}", err);
            err
        })?;
        println!("a1");
        // Check if the request was successful
        if response.status().is_success() {
            println!("success");
            Ok(true)
        } else {
            println!("failure");
            Ok(false)
        }
    }
}