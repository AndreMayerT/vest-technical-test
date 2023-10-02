use async_graphql::*;
use crate::graphql::types::*;
use tokio_postgres::NoTls;
use reqwest::Client;

pub struct Query;

#[Object]
impl Query {
    async fn portfolio(&self, ctx: &Context<'_>) -> FieldResult<Vec<Holding>> {
        let (client, connection) = tokio_postgres::connect("host=db user=user dbname=mydb password=aaabbbccc", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {:?}", e);
            }
        });

        let nasdaq_client = ctx.data::<Client>().expect("reqwest::Client not found in Context");


        let rows = client.query("SELECT * FROM holdings", &[]).await?;
        
        let mut holdings = Vec::new();
        for row in rows {
            let symbol: String = row.get("symbol");
            let quantity: i32 = row.get("quantity");
            let average_cost_per_share: f64 = row.get("average_cost_per_share");

            
            // API call to NASDAQ API to get current price and reference prices
            
            let url = format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol);
            let response = nasdaq_client.get(&url).send().await?;
            if response.status().is_success() {
                let api_response: serde_json::Value = response.json().await?;
                if api_response["status"]["rCode"].as_u64() != Some(200) {
                    return Err(FieldError::new(
                        "Error fetching data from NASDAQ API")
                    );
                }

                let current_price = api_response["data"]["primaryData"]["lastSalePrice"]
                    .as_str().unwrap_or_default()
                    .trim_start_matches("$")
                    .parse::<f64>()
                    .unwrap();

                let price_range_str = api_response["data"]["keyStats"]["dayrange"]["value"].as_str();
                
                let lowest_price: f64 = price_range_str
                    .and_then(|s| s.split('-').nth(0))
                    .unwrap()
                    .trim()
                    .parse::<f64>()
                    .unwrap();
                
                let highest_price: f64 = price_range_str
                    .and_then(|s| s.split('-').nth(1))
                    .unwrap()
                    .trim()
                    .parse::<f64>()
                    .unwrap();

                let average_price = (highest_price + lowest_price) / 2 as f64;
                
                let profit_loss = ((current_price - average_cost_per_share) / average_cost_per_share) * 100.0;

                let profit_loss_percentage = format!("{:.2}%", profit_loss);

                let current_value =  current_price * quantity as f64;

                let reference_prices = ReferencePrices { 
                    lowest_price: lowest_price, 
                    highest_price: highest_price, 
                    average_price: average_price 
                };


                holdings.push(Holding {
                    symbol,
                    profit_loss_percentage, // Calculated value
                    share_held: quantity,
                    current_value, // Calculated value
                    reference_prices, // Calculated value
                });
            } else {
                return Err(FieldError::new(
                    "Error fetching data from NASDAQ API")
                );
            }
        }
        
        Ok(holdings)
    }

    async fn historical_price(&self, ctx: &Context<'_>, symbol: String) -> FieldResult<Vec<HourlyPricePoint>> {
        let url = format!("https://api.nasdaq.com/api/quote/{}/chart?assetclass=stocks", symbol);

        let client = ctx.data::<Client>().expect("reqwest::Client not found in Context");

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let api_response: ApiResponse = response.json().await?;
            
            let mut hourly_data: Vec<HourlyPricePoint> = Vec::new();
            
            for point in api_response.data.chart {
                
                // Extract the hour and select the price at that exact hour
                let datetime_str = point.z.dateTime; 
                let value = point.z.value.parse().unwrap_or(0.0);
                
                let parts: Vec<&str> = datetime_str.split_whitespace().collect();
                if let Some(time) = parts.get(0) {
                    let time_parts: Vec<&str> = time.split(':').collect();
                    let minute = time_parts.get(1).cloned().unwrap();

                    if minute == "00" {
                        hourly_data.push(HourlyPricePoint { hour: datetime_str, price: value });
                    }
                }
            }
            
            Ok(hourly_data)
        } else {
            return Err(FieldError::new("Error fetching data from NASDAQ API"));
        }
    }
}