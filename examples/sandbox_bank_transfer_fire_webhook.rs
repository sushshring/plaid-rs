use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let webhook = "your webhook";
    let response = client
        .sandbox_bank_transfer_fire_webhook(webhook)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
