use crate::repoistories::{wallet_owner_stable::WalletOwnerStableRepositoy, WalletOwnerRepository};

pub fn execute(repo: &WalletOwnerStableRepositoy) -> u64 {
    repo.count_wallet()
}
