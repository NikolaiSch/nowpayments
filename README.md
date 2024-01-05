# NowPayments Rust Api
Free, No ID verification Crypto2Crypto payment provider

## Usage
```rust
// setting up client
let mut client = NPClient::new(config.api);
client.set_auth(config.email, config.password);

// creating options for your request
let payment = PaymentOpts::new(100, "GBP", "BTC", "http://google.com/", &id, "test order");

// have to create a new JWT every 5 minutes for nowpayments
client.authenticate().await?;
let order = client.create_payment(payment).await?;

// using payment response to get status
let status = client.get_payment_status(order.payment_id).await?;
```
