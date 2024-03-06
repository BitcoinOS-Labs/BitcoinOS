// use amplify::Display;
// use bdk::{bitcoin::Txid, Balance, BlockTime, TransactionDetails};
// use serde::{Deserialize, Serialize};
// use zeroize_derive::{Zeroize, ZeroizeOnDrop};

// #[derive(Debug, Deserialize, Serialize)]
// pub struct WalletData {
//     pub address: String,
//     pub balance: Balance,
//     pub transactions: Vec<WalletTransaction>,
//     pub utxos: Vec<String>,
// }

// #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
// pub struct WalletTransaction {
//     pub txid: Txid,
//     pub received: u64,
//     pub sent: u64,
//     pub fee: Option<u64>,
//     pub confirmed: bool,
//     pub confirmation_time: Option<BlockTime>,
//     pub vsize: usize,
//     pub fee_rate: f32,
// }

// #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
// pub struct TransactionData {
//     pub txs: TransactionDetails,
//     pub vsize: usize,
//     pub fee_rate: f32,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop, Display, Default)]
// #[display(inner)]
// pub struct SecretString(pub String);

// pub struct Private

// pub struct Transaction {}

// pub enum Asset {
//     Atom,
//     Ordi,
//     #[allow(clippy::upper_case_acronyms)]
//     RGB,
// }
