use serde::{Deserialize, Serialize};
use zeroize_derive::{Zeroize, ZeroizeOnDrop};

#[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct RgbData {
    pub asset_descriptor_xprv: String,
    pub uda_descriptor_xprv: String,
}
