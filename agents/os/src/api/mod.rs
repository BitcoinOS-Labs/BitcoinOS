pub mod count_wallet;
pub mod create_wallet;

#[ic_cdk::update]
pub fn create_wallet() -> bool {
    let owner = ic_cdk::caller();
    let canister_id = ic_cdk::api::id();
    let created_at = ic_cdk::api::time();

    create_wallet::execute(owner, canister_id, created_at)
}

#[ic_cdk::query]
pub fn count_wallet() -> u64 {
    count_wallet::execute()
}
