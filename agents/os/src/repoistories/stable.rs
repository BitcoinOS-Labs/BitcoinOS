use crate::{domain::WalletOwner, error::Error, WalletOwnerStable};

use super::Repository;

pub struct StableRepositoy<'a> {
    pub owners: &'a mut WalletOwnerStable,
}

impl<'a> Repository for StableRepositoy<'a> {
    fn insert_wallet_owner(
        &mut self,
        owner: candid::Principal,
        canister_id: candid::Principal,
        created_at: u64,
    ) -> Result<WalletOwner, Error> {
        if self.owners.contains_key(&canister_id) {
            Err(Error::AlreadyExists)
        } else {
            let wallet_owner = WalletOwner {
                canister_id,
                owner,
                created_at,
            };
            self.owners
                .insert(canister_id, wallet_owner)
                .ok_or(Error::Unknown)
        }
    }
}
