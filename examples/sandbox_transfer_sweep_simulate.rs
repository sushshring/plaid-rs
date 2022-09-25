use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client.sandbox_transfer_sweep_simulate().send().await.unwrap();
    println!("{:#?}", response);
}
