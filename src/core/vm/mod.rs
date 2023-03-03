// VM should be a trait and should be modular so that the user can put in their own VM.
// This will allow for more flexibility and will allow for more VMs to be added in the future.
use anyhow::Result;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub mod bytecode_vm;

use super::state::State;

pub type DynVM = Box<dyn VM>;

pub trait VM: Debug + DynClone + Send + Sync {
    fn execute(&self, state: &dyn State, code: &[u8]) -> Result<Vec<u8>>;
}

dyn_clone::clone_trait_object!(VM);
