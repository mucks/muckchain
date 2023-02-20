use crate::core::{DynDecoder, DynEncoder};

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub encoder: DynEncoder,
    pub decoder: DynDecoder,
}
