use serde::{Deserialize, Serialize};

use super::Hash;

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum McError {
    #[error("Block {0} already exists!")]
    BlockAlreadyExists(Hash),
}
