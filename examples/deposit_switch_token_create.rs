use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let deposit_switch_id = "your deposit switch id";
    let response = client
        .deposit_switch_token_create(deposit_switch_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
