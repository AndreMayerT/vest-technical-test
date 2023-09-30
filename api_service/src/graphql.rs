use crate::services::order_service::*;
use async_graphql::*;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use tokio_postgres::NoTls;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(InputObject, Serialize, Deserialize)]
pub struct OrderInput {
    symbol: String,
    quantity: i32,
    order_type: OrderType,
    value: Option<f64>
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ReferencePrices {
    lowest_price: f64,
    highest_price: f64,
    average_price: f64
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Holding {
    symbol: String,
    profit_loss_percentage: String,
    share_held: i32,
    current_value: f64,
    reference_prices: ReferencePrices
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn place_order(&self, ctx: &Context<'_>,mut input: OrderInput) -> Result<String> {
        // Validate stock symbol
        let client = ctx.data::<Client>().expect("reqwest::Client not found in Context");

        let operation_price = make_order(&input.symbol, input.quantity, client).await?;
        if operation_price == 0.00 {
            return Ok("Invalid symbol".to_string());
        }

        input.value = Some(operation_price);

        // Send order to Kafka
        let message = send_order_to_kafka(&input).await?;
        println!("{:?}", message);


        Ok("Order sent".to_string())
        
    }
}

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
}

pub fn create_schema() -> Schema<Query, Mutation, EmptySubscription> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .expect("Failed to build reqwest client");

    Schema::build(Query, Mutation, EmptySubscription)
        .data(client)
        .finish()
}