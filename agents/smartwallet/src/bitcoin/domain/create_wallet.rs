use bdk::{database::MemoryDatabase, Wallet};

pub fn execute() -> Wallet<MemoryDatabase> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_wallet_otherwise() {
        let _wallet = execute();
    }
}
