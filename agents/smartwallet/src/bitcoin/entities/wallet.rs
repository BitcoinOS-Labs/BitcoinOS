use bdk::{bitcoin::Txid, Balance, BlockTime, TransactionDetails};
use serde::{Deserialize, Serialize};
use zeroize_derive::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Deserialize, Serialize)]
pub struct BitcoinData {
    pub address: String,
    pub balance: Balance,
    pub transactions: Vec<BitcoinTransaction>,
    pub utxos: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BitcoinTransaction {
    pub txid: Txid,
    pub received: u64,
    pub sent: u64,
    pub fee: Option<u64>,
    pub confirmed: bool,
    pub confirmation_time: Option<BlockTime>,
    pub vsize: usize,
    pub fee_rate: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BitcoinTransactionData {
    pub txs: TransactionDetails,
    pub vsize: usize,
    pub fee_rate: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct BitcoinPrivateData {
    pub xprvkh: String,
    pub descriptor_xprv: String,
    pub change_descriptor_xprv: String,
}
