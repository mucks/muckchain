pub mod contract_state;

use dyn_clone::DynClone;
use std::fmt::Debug;

pub type DynState = Box<dyn State>;

pub trait State: Debug + DynClone + Send + Sync {}

dyn_clone::clone_trait_object!(State);
