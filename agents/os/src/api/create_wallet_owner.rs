use candid::Principal;

use crate::{
    domain::WalletOwner, error::Error, repoistories::insert_wallet_owner::StableRepositoy,
    services, WALLET_OWNER,
};

pub fn serve(
    owner: Principal,
    canister_id: Principal,
    created_at: u64,
) -> Result<WalletOwner, Error> {
    WALLET_OWNER.with(|w| {
        let mut repo = StableRepositoy {
            owners: &mut w.borrow_mut(),
        };
        services::insert_wallet_owner::execute(&mut repo, owner, canister_id, created_at)
    })
}
