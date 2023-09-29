use crate::services::order_service::*;


use async_graphql::*;
use serde::{Serialize, Deserialize};

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
    value: Option<f32>
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn place_order(&self, mut input: OrderInput) -> Result<String> {
        // Validate stock symbol
        let operation_price = make_order(&input.symbol, input.quantity).await?;
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
    async fn dummy(&self, _ctx: &Context<'_>) -> FieldResult<&str> {
        Ok("I am a dummy query")
    }
}