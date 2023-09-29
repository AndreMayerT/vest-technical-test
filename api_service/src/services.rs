

pub mod order_service {
    use reqwest::{Error, Client};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ApiResponse {
        status: ApiStatus,
    }

    #[derive(Deserialize, Debug)]
    struct ApiStatus {
        rCode: u16,
    }

    pub async fn validate_symbol(symbol: &str) -> Result<bool, Error> {
        let client = Client::builder()
        .use_rustls_tls() 
        .build()?;

        let url = format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol);

        // Send a Get request to the NASDAQ API
        let response = client.get(&url).send().await?;
        // Check if the request was successful
        if response.status().is_success() {
            let api_response: ApiResponse = response.json().await?;
            if api_response.status.rCode == 200 {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}