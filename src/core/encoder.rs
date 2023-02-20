use std::fmt::Debug;

use anyhow::Result;
use erased_serde::Serialize;
// use serde::{de::DeserializeOwned, Serialize};

pub type DynEncoder = Box<dyn Encoder>;

pub trait Encoder: EncoderClone + Debug + Send + Sync {
    fn encode(&self, val: &dyn Serialize) -> Result<Vec<u8>>;
}

pub trait EncoderClone {
    fn clone_box(&self) -> Box<dyn Encoder>;
}

impl<T> EncoderClone for T
where
    T: 'static + Encoder + Clone,
{
    fn clone_box(&self) -> Box<dyn Encoder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Encoder> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct JsonEncoder;

impl Encoder for JsonEncoder {
    fn encode(&self, val: &dyn Serialize) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(val)?;
        Ok(bytes)
    }
}
