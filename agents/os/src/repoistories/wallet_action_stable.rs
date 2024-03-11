use std::cell::RefCell;

use crate::{domain::WalletAction, error::Error, WalletActionStable};

use super::WalletActionRepository;

pub struct WalletActionStableRepository<'a> {
    pub actions: &'a RefCell<WalletActionStable>,
}

impl<'a> WalletActionStableRepository<'a> {
    // pub fn new(actions: &RefCell<WalletActionStable>) -> Self {
    //     Self<'a> {
    //         actions
    //     }
    // }

    // pub fn borrow(&self) -> &RefCell<WalletActionStable> {
    //     self.actions
    // }
}

impl<'a> WalletActionRepository for WalletActionStableRepository<'a> {
    fn append_wallet_action(
        &self,
        operator: candid::Principal,
        action: crate::domain::Action,
        op_time: u64,
    ) -> Result<u64, Error> {
        let wallet_action = WalletAction {
            operator,
            action,
            op_time,
        };

        self.actions
            .borrow()
            .append(&wallet_action)
            .map_err(|_| Error::WriteError)
    }
}
