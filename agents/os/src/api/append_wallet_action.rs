use candid::Principal;

use crate::{
    domain::Action, error::Error, repoistories::wallet_action_stable::WalletActionStableRepository,
    services, WALLET_CREATED_LOG,
};

pub fn serve(operator: Principal, action: Action, op_time: u64) -> Result<u64, Error> {
    WALLET_CREATED_LOG.with(|w| {
        let repo = WalletActionStableRepository { actions: w };
        services::append_wallet_action::execute(repo, operator, action, op_time)
    })
}
