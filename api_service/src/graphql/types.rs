use async_graphql::*;
use serde::{Serialize, Deserialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(InputObject, Serialize, Deserialize)]
pub struct OrderInput {
    pub symbol: String,
    pub quantity: i32,
    order_type: OrderType,
    pub value: Option<f64>
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ReferencePrices {
    pub lowest_price: f64,
    pub highest_price: f64,
    pub average_price: f64
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Holding {
    pub symbol: String,
    pub profit_loss_percentage: String,
    pub share_held: i32,
    pub current_value: f64,
    pub reference_prices: ReferencePrices
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct HourlyPricePoint {
    pub hour: String,
    pub price: f64 
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: ApiData,
}

#[derive(Debug, Deserialize)]
pub struct ApiData {
    pub chart: Vec<ChartPoint>,
}

#[derive(Debug, Deserialize)]
pub struct ChartPoint {
    pub z: HistoricalPricePoint,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalPricePoint {
    pub dateTime: String,
    pub value: String,
}