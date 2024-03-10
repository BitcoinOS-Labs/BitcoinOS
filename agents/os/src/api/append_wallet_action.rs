use candid::Principal;

use crate::{
    domain::{Action, WalletAction},
    error::Error,
    repoistories::{appen_wallet_action, insert_wallet_owner},
    WALLET_CREATED_LOG, WALLET_OWNER,
};

pub fn execute(operator: Principal, action: Action, op_time: u64) -> Result<bool, Error> {
    let wallet_action = WalletAction {
        operator,
        action,
        op_time,
    };

    // WALLET_CREATED_LOG.with(|w| appen_wallet_action::execute(&mut w.borrow_mut(), wallet_action))
    todo!()
}
