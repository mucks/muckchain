// VM should be a trait and should be modular so that the user can put in their own VM.
// This will allow for more flexibility and will allow for more VMs to be added in the future.

use anyhow::Result;
use dyn_clone::DynClone;
use std::fmt::Debug;

pub mod bytecode_vm;

use super::state::DynState;
pub type DynVM = Box<dyn VM>;

#[async_trait::async_trait]
pub trait VM: Debug + DynClone + Send + Sync {
    async fn execute(&mut self, state: &DynState, code: &[u8]) -> Result<()>;
}

dyn_clone::clone_trait_object!(VM);
