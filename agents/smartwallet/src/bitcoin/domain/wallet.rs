use std::sync::{Arc, Mutex};

use bdk::{database::MemoryDatabase, Wallet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitcoinWalletError {
    /// BDK error
    #[error(transparent)]
    BdkError(#[from] bdk::Error),
}

pub fn get_wallet_(
    _descriptor: &str,
    _change_descriptor: Option<&str>,
) -> Result<Arc<Mutex<Wallet<MemoryDatabase>>>, BitcoinWalletError> {
    todo!()
}
