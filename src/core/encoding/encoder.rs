use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait Encodable: Send + Sync {}

use anyhow::Result;
use dyn_clone::DynClone;

pub type DynEncoder = Box<dyn Encoder>;

pub trait Encoder: Debug + DynClone + Send + Sync {
    fn encode(&self, val: &dyn Encodable) -> Result<Vec<u8>>;
}
dyn_clone::clone_trait_object!(Encoder);

macro_rules! encodable {
    ($a:ident) => {
        #[typetag::serde]
        impl Encodable for $a {}
    };
}

pub(crate) use encodable;
