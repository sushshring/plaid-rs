use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let bank_transfer_id = "your bank transfer id";
    let event_type = "your event type";
    let response = client
        .sandbox_bank_transfer_simulate(bank_transfer_id, event_type)
        .failure_reason(BankTransferFailure {
            description: Some("your description".to_owned()),
            ach_return_code: Some("your ach return code".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
