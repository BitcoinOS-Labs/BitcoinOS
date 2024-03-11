pub mod append_wallet_action;
pub mod count_wallet;
pub mod create_wallet_owner;
pub mod get_wallet_action;
pub mod list_wallet;

use crate::{
    domain::{Action, WalletAction, WalletOwner},
    error::Error,
};

/// Create a smart wallet canister, log the action, and store the wallet owner info
#[ic_cdk::update]
pub fn create_wallet() -> Result<WalletOwner, Error> {
    let owner = ic_cdk::caller();
    let canister_id = ic_cdk::api::id();
    let created_at = ic_cdk::api::time();

    // TODO: create smart wallet canister
    // create_wallet::

    append_wallet_action::serve(owner, Action::Create, created_at)?;

    create_wallet_owner::serve(owner, canister_id, created_at)
}

#[ic_cdk::query]
pub fn count_wallet() -> u64 {
    count_wallet::serve()
}

#[ic_cdk::query]
pub fn list_wallet() -> Vec<WalletOwner> {
    list_wallet::serve()
}

#[ic_cdk::query]
pub fn get_wallet_action(idx: u64) -> Option<WalletAction> {
    get_wallet_action::serve(idx)
}
