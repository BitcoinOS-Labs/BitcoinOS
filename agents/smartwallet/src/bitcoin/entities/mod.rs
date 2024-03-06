pub mod wallet;

use amplify::Display;
use serde::{Deserialize, Serialize};
use zeroize_derive::{Zeroize, ZeroizeOnDrop};

#[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop, Display, Default)]
#[display(inner)]
pub struct SecretString(pub String);
