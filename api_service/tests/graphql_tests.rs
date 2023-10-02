use api_service::graphql::create_schema;
use tokio::runtime::Runtime;


#[test]
fn test_place_order() {
    let schema = create_schema();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let res = schema.execute(
            r#"
            mutation {
                placeOrder(input: {
                    symbol: "AAPL",
                    quantity: 1,
                    orderType: BUY,
                    value: 150.0
                })
            }
            "#,
        ).await;

        assert!(res.errors.is_empty(), "Expected no errors, got: {:?}", res.errors);
        assert_eq!(res.data, async_graphql::Value::String("Order sent".to_string()));

        let res = schema.execute(
            r#"
            mutation {
                placeOrder(input: {
                    symbol: "AAPL",
                    quantity: 2,
                    orderType: SELL,
                    value: 150.0
                })
            }
            "#,
        ).await;

        assert!(res.errors.is_empty(), "Expected no errors, got: {:?}", res.errors);
        assert_eq!(res.data, async_graphql::Value::String("Order sent".to_string()));

        let res = schema.execute(
            r#"
            mutation {
                placeOrder(input: {
                    symbol: "asassa",
                    quantity: 1,
                    orderType: BUY,
                    value: 150.0
                })
            }
            "#,
        ).await;

        assert!(res.errors.is_empty(), "Expected no errors, got: {:?}", res.errors);
        assert_eq!(res.data, async_graphql::Value::String("Invalid symbol".to_string()));
    });
}


#[test]
fn test_portfolio() {
    let schema = create_schema();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let res = schema.execute(
            r#"
            query {
                portfolio {
                    symbol
                    profitLossPercentage
                    shareHeld
                    currentValue
                    referencePrices {
                        lowestPrice
                        highestPrice
                        averagePrice
                    }
                }
            }
            "#,
        ).await;

        assert!(res.errors.is_empty(), "Expected no errors, got: {:?}", res.errors);
    });
}

#[test]
fn test_historical_price() {
    let schema = create_schema();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let res = schema.execute(
            r#"
            query {
                historicalPrice(symbol: "AAPL") {
                    hour
                    price
                }
            }
            "#,
        ).await;

        assert!(res.errors.is_empty(), "Expected no errors, got: {:?}", res.errors);
    });
}