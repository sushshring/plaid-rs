use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client.categories_get().send().await.unwrap();
    println!("{:#?}", response);
}
