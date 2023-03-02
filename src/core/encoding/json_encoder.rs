use super::encoder::*;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct JsonEncoder;

impl Encoder for JsonEncoder {
    fn encode(&self, val: &dyn Encodable) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(val)?;
        Ok(bytes)
    }
}
