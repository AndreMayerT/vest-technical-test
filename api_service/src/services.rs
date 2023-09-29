

pub mod order_service {
    use reqwest::{Error, Client};

    pub async fn validate_symbol(symbol: &str) -> Result<bool, Error> {
        let client = Client::builder()
        .use_rustls_tls() 
        .build()?;

        let url = format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol);

        // Send a Get request to the NASDAQ API
        let response = client.get(url).send().await?;
        
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