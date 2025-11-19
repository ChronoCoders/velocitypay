pub mod user;
pub mod transaction;
pub mod mint_request;
pub mod burn_request;
pub mod kyc;
pub mod response;

// Type validation to ensure all model structs are properly defined
// These are constructed by serde during JSON deserialization
#[allow(dead_code)]
fn _validate_models_are_constructible() {
    use chrono::Utc;
    use uuid::Uuid;

    // Validate transaction models
    let _tx = transaction::Transaction {
        id: Uuid::new_v4(),
        from_address: String::new(),
        to_address: String::new(),
        amount: String::new(),
        fee: String::new(),
        transaction_hash: None,
        block_number: None,
        status: transaction::TransactionStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let _tx_resp = transaction::TransactionResponse {
        id: Uuid::new_v4(),
        from_address: String::new(),
        to_address: String::new(),
        amount: String::new(),
        fee: String::new(),
        transaction_hash: None,
        block_number: None,
        status: transaction::TransactionStatus::Pending,
        created_at: Utc::now(),
    };

    // Validate mint request models
    let _mint = mint_request::MintRequest {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        wallet_address: String::new(),
        amount: String::new(),
        bank_reference: String::new(),
        status: mint_request::MintRequestStatus::Pending,
        chain_request_id: None,
        approved_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let _mint_resp = mint_request::MintRequestResponse {
        id: Uuid::new_v4(),
        wallet_address: String::new(),
        amount: String::new(),
        bank_reference: String::new(),
        status: mint_request::MintRequestStatus::Pending,
        chain_request_id: None,
        created_at: Utc::now(),
    };

    // Validate burn request models
    let _burn = burn_request::BurnRequest {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        wallet_address: String::new(),
        amount: String::new(),
        bank_account: String::new(),
        status: burn_request::BurnRequestStatus::Pending,
        chain_request_id: None,
        approved_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let _burn_resp = burn_request::BurnRequestResponse {
        id: Uuid::new_v4(),
        wallet_address: String::new(),
        amount: String::new(),
        bank_account: String::new(),
        status: burn_request::BurnRequestStatus::Pending,
        chain_request_id: None,
        created_at: Utc::now(),
    };

    // Validate KYC models
    let _kyc = kyc::KYCSubmission {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        wallet_address: String::new(),
        document_hash: String::new(),
        full_name: String::new(),
        date_of_birth: Utc::now(),
        country: String::new(),
        status: kyc::KYCStatus::Pending,
        verified_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let _kyc_resp = kyc::KYCResponse {
        id: Uuid::new_v4(),
        wallet_address: String::new(),
        status: kyc::KYCStatus::Pending,
        created_at: Utc::now(),
    };
}
