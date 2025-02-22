use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .bank_transfer_event_list()
        .start_date("your start date")
        .end_date("your end date")
        .bank_transfer_id("your bank transfer id")
        .account_id("your account id")
        .bank_transfer_type("your bank transfer type")
        .event_types(&["your event types"])
        .count(1)
        .offset(1)
        .origination_account_id("your origination account id")
        .direction("your direction")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
