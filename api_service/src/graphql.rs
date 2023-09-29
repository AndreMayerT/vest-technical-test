use crate::services::order_service::*;

use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(InputObject)]
pub struct OrderInput {
    symbol: String,
    quantity: i32,
    order_type: OrderType,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn place_order(&self, input: OrderInput) -> Result<String> {
        // Validate stock symbol
        let is_valid = validate_symbol(&input.symbol).await?;
        println!("{}", is_valid);
        if !is_valid {
            return Ok("Invalid symbol".to_string());
        }

        // Send order to Kafka
        // send_order_to_kafka(&input).await?;

        // Return a simple message indicating that the order has been sent
        Ok("Order sent".to_string())
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn dummy(&self, _ctx: &Context<'_>) -> FieldResult<&str> {
        Ok("I am a dummy query")
    }
}