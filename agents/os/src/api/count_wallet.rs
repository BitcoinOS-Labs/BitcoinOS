use crate::WALLET_OWNER;

pub fn execute() -> u64 {
    WALLET_OWNER.with(|w| w.borrow().len())
}
