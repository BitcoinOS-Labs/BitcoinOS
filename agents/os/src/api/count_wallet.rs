use crate::{
    repoistories::wallet_owner_stable::WalletOwnerStableRepositoy, services, WALLET_OWNER,
};

pub fn serve() -> u64 {
    WALLET_OWNER.with(|w| {
        let repo = WalletOwnerStableRepositoy {
            owners: &mut w.borrow_mut(),
        };
        services::count_wallet::execute(&repo)
    })
}
