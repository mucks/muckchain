use anyhow::Result;
use dyn_clone::DynClone;
use std::{any::Any, fmt::Debug};

#[typetag::serde(tag = "type")]
pub trait Decodable: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub fn decode<T: Decodable + Clone + 'static>(
    decoder: &Box<dyn Decoder>,
    data: &[u8],
) -> Result<T> {
    let decodable: Box<dyn Decodable> = decoder.decode(data)?;
    let decodable = decodable.as_any().downcast_ref::<T>().unwrap();
    Ok(decodable.clone())
}

pub type DynDecoder = Box<dyn Decoder>;

pub trait Decoder: Debug + DynClone + Send + Sync {
    fn decode(&self, data: &[u8]) -> Result<Box<dyn Decodable>>;
}

dyn_clone::clone_trait_object!(Decoder);
