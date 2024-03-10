use candid::Principal;

use crate::{domain::WalletOwner, error::Error};

pub mod appen_wallet_action;
pub mod stable;

pub trait Repository {
    fn insert_wallet_owner(
        &mut self,
        owner: Principal,
        canister_id: Principal,
        created_at: u64,
    ) -> Result<WalletOwner, Error>;
}
