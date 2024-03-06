use super::entities::Wallet;

pub fn execute() -> Wallet {
    Wallet::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_wallet_otherwise() {
        let wallet = execute();
    }
}
