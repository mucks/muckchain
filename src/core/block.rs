use crate::model::MyHash;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{BlockHeader, Encoder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
}
