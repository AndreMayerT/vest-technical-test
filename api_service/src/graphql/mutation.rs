use async_graphql::*;
use crate::services::order_service::*;
use crate::graphql::types::*;
use reqwest::Client;

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