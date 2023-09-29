

pub mod order_service {
    use reqwest::{Error, Client};
    use serde::Deserialize;
    use rdkafka::config::ClientConfig;
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::error::KafkaError;
    use std::time::Duration;
    use crate::graphql::OrderInput;

    #[derive(Deserialize, Debug)]
    struct ApiResponse {
        status: ApiStatus,
        data: PrimaryData
    }

    #[derive(Deserialize, Debug)]
    struct PrimaryData {
        primaryData: AskPrice,
    }

    #[derive(Deserialize, Debug)]
    struct AskPrice {
        askPrice: String,
    }

    #[derive(Deserialize, Debug)]
    struct ApiStatus {
        rCode: u16,
    }

    pub async fn make_order(symbol: &str, quantity: i32) -> Result<f32, Error> {
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
                let price_str = api_response.data.primaryData.askPrice;
                let price_without_dollar = price_str.trim_start_matches('$');
                if let Ok(price) = price_without_dollar.parse::<f32>() {
                    let operation_price = price * (quantity as f32);
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
}
