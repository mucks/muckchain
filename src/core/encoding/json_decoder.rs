use super::decoder::{Decodable, Decoder};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct JsonDecoder;

impl Decoder for JsonDecoder {
    fn decode(&self, data: &[u8]) -> Result<Box<dyn Decodable>> {
        Ok(serde_json::from_slice(data)?)
    }
}
