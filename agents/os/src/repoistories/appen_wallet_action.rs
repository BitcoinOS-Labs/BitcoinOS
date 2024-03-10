use crate::{domain::WalletAction, error::Error, WalletActionStable};

pub fn execute(
    wallet_actions: &mut WalletActionStable,
    wallet_action: &WalletAction,
) -> Result<u64, Error> {
    wallet_actions
        .append(wallet_action)
        .map_err(|_| Error::WriteError)
}
