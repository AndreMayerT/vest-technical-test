
use rdkafka::consumer::{Consumer, StreamConsumer, DefaultConsumerContext};
use rdkafka::config::ClientConfig;
use tokio_stream::StreamExt; 
use rdkafka::message::Message;
use tokio_postgres::NoTls;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize)]
struct Order {
    symbol: String,
    quantity: i32,
    order_type: OrderType,
    value: f64,
}


#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    // Setup Kafka Consumer
    let consumer: StreamConsumer<DefaultConsumerContext> = ClientConfig::new()
        .set("bootstrap.servers", "kafka:9092")
        .set("group.id", "order_processor_group")
        .create()
        .expect("Consumer creation failed");
    
    consumer
        .subscribe(&["orders"])
        .expect("Can't subscribe to specified topics");

    // Process messages from Kafka
    let mut message_stream = consumer.stream();
    loop {
        while let Some(message) = message_stream.next().await {
            match message {
                Ok(message) => {
                    let payload = message.payload().unwrap_or_default();
                    match serde_json::from_slice::<Order>(payload) {
                        Ok(order) => {
                            if let Err(e) = process_order(order).await {
                                println!("Error processing order: {:?}", e);
                            }
                        }
                        Err(e) => println!("Error deserializing message payload: {:?}", e),
                    }
                }
                Err(err) => println!("Error receiving message: {:?}", err),
            }
        }
    }
}

async fn process_order(order: Order) -> Result<(), Box<dyn Error>> {
    // Print the order to the console
    println!("Received Order: {:?}", order);

    // Calculate the price per share
    let price_per_share = order.value / order.quantity as f64;

    // Connect to the database
    let (client, connection) = tokio_postgres::connect("host=db user=user dbname=mydb password=aaabbbccc", NoTls).await?;
    
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {:?}", e);
        }
    });
    
    // Process the order
    match order.order_type {
        OrderType::Buy => {

            // Check if a holding already exists for the given symbol
            let row = client.query_one("SELECT quantity, average_cost_per_share, total_cost FROM holdings WHERE symbol = $1", &[&order.symbol]).await;
            
            match row {
                Ok(existing_holding) => {
                    // Update the existing holding
                    let new_quantity = existing_holding.get::<_, i32>("quantity") + order.quantity;
                    let new_total_cost = existing_holding.get::<_, f64>("total_cost") + order.value;
                    let new_average_cost_per_share = new_total_cost / new_quantity as f64;
                    
                    client.execute(
                        "UPDATE holdings SET quantity = $1, average_cost_per_share = $2, total_cost = $3 WHERE symbol = $4",
                        &[&new_quantity, &new_average_cost_per_share, &new_total_cost, &order.symbol]
                    ).await?;
                }
                Err(_) => {
                    // Insert a new holding
                    client.execute(
                        "INSERT INTO holdings (symbol, quantity, average_cost_per_share, total_cost) VALUES ($1, $2, $3, $4)",
                        &[&order.symbol, &order.quantity, &price_per_share, &order.value]
                    ).await?;
                }
            }
        }
        OrderType::Sell => {

            // Fetch the holding from the database
            let row = client.query_one("SELECT quantity, total_cost FROM holdings WHERE symbol = $1", &[&order.symbol]).await;
    
            match row {
                Ok(existing_holding) => {
                    let held_quantity: i32 = existing_holding.get("quantity");
                    let total_cost: f64 = existing_holding.get("total_cost");
                    
                    // Check if there are enough shares to sell
                    if held_quantity < order.quantity {
                        return Err("Not enough shares to sell".into())
                    }
                    
                    // Update the holdings
                    let new_quantity = held_quantity - order.quantity;
                    let new_total_cost = total_cost - (order.value as f64);
                    
                    if new_quantity > 0 {
                        client.execute(
                            "UPDATE holdings SET quantity = $1, total_cost = $2 WHERE symbol = $3",
                            &[&new_quantity, &new_total_cost, &order.symbol]
                        ).await?;
                    } else {
                        // If no more shares are left, delete the holding
                        client.execute("DELETE FROM holdings WHERE symbol = $1", &[&order.symbol]).await?;
                    }
                }

                Err(_) => {
                    // If no holding exists for the given symbol, return an error
                    return Err("No shares available to sell".into());
                }
            }
        }
    }

    Ok(())
}