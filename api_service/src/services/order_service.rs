use reqwest::{Error, Client};
use serde_json::Value;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaError;
use std::time::Duration;
use crate::graphql::OrderInput;

pub async fn make_order(symbol: &str, quantity: i32, client: &Client) -> Result<f64, Error> {
    
    let url = format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol);

    // Send a Get request to the NASDAQ API
    let response = client.get(&url).send().await?;

    // Check if the request was successful
    if response.status().is_success() {
        let api_response: Value = response.json().await?;
        if api_response["status"]["rCode"].as_u64() == Some(200) {

            // Get the current price of the share
            let price_str = api_response["data"]["primaryData"]["lastSalePrice"].as_str().unwrap_or_default();
            let price_without_dollar = price_str.trim_start_matches('$');

            // Calculate the total cost of operation
            if let Ok(price) = price_without_dollar.parse::<f64>() {
                let operation_price = price * (quantity as f64);
                println!("{}", operation_price);
                Ok(operation_price)
            } else {
                Ok(0.00)
            }
        } else {
            Ok(0.00)
        }
    } else {
        Ok(0.00)
    }
}


pub async fn send_order_to_kafka(order: &OrderInput) -> Result<(), KafkaError>{

    // Create Kafka producer
    let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "kafka:9092")
    .set("message.timeout.ms", "5000")
    .create()?;

    // Convert order to json
    let order_json = match serde_json::to_string(order) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Failed to serialize OrderInput: {}", e);
            return Err(KafkaError::MessageProduction(rdkafka::types::RDKafkaErrorCode::Application));
        }
    };

    // Send the order to Kafka topic
    let produce_future = producer.send(
    FutureRecord::to("orders")
        .key("")
        .payload(&order_json),
    Duration::from_secs(0),
    );

    match produce_future.await {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => println!("Error: {:?}", e),
    }

    Ok(())
}