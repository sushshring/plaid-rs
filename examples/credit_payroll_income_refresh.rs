use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .credit_payroll_income_refresh()
        .user_token("your user token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
