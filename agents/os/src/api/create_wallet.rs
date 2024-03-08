use candid::Principal;

use crate::{domain::WalletOwner, WALLET_OWNER};

pub fn execute(owner: Principal, canister_id: Principal, created_at: u64) -> bool {
    let wallet_owner = WalletOwner {
        canister_id,
        owner,
        created_at,
    };

    WALLET_OWNER.with(|w| w.borrow_mut().insert(canister_id, wallet_owner).is_some())
}
