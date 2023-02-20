use anyhow::Result;
use std::{any::Any, fmt::Debug};

#[typetag::serde(tag = "type")]
pub trait Decodable {
    fn as_any(&self) -> &dyn Any;
}

pub fn decode<T: Decodable + Clone + 'static>(decoder: Box<dyn Decoder>, data: &[u8]) -> Result<T> {
    let decodable: Box<dyn Decodable> = decoder.decode(data)?;
    let decodable = decodable.as_any().downcast_ref::<T>().unwrap();
    Ok(decodable.clone())
}

pub type DynDecoder = Box<dyn Decoder>;

pub trait Decoder: DecoderClone + Debug + Send + Sync {
    fn decode(&self, data: &[u8]) -> Result<Box<dyn Decodable>>;
}

pub trait DecoderClone {
    fn clone_box(&self) -> Box<dyn Decoder>;
}

impl<T> DecoderClone for T
where
    T: 'static + Decoder + Clone,
{
    fn clone_box(&self) -> Box<dyn Decoder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Decoder> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct JsonDecoder;

impl Decoder for JsonDecoder {
    fn decode(&self, data: &[u8]) -> Result<Box<dyn Decodable>> {
        Ok(serde_json::from_slice(data)?)
    }
}
