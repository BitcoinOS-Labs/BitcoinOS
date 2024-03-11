use candid::Principal;

use crate::{
    domain::WalletOwner, error::Error,
    repoistories::wallet_owner_stable::WalletOwnerStableRepositoy, services, WALLET_OWNER,
};

pub fn serve(
    owner: Principal,
    canister_id: Principal,
    created_at: u64,
) -> Result<WalletOwner, Error> {
    WALLET_OWNER.with(|w| {
        let mut repo = WalletOwnerStableRepositoy { owners: w };
        services::insert_wallet_owner::execute(&mut repo, owner, canister_id, created_at)
    })
}
