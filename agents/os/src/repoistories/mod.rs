use candid::Principal;

use crate::{
    domain::{Action, WalletOwner},
    error::Error,
};

pub mod wallet_action_stable;
pub mod wallet_owner_stable;

pub trait WalletOwnerRepository {
    fn insert_wallet_owner(
        &mut self,
        owner: Principal,
        canister_id: Principal,
        created_at: u64,
    ) -> Result<WalletOwner, Error>;

    fn count_wallet(&self) -> u64;
}

pub trait WalletActionRepository {
    fn append_wallet_action(
        &self,
        operator: Principal,
        action: Action,
        op_time: u64,
    ) -> Result<u64, Error>;
}
