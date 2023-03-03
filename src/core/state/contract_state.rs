use super::State;

#[derive(Debug, Clone)]
pub struct ContractState {}

impl ContractState {
    pub fn new() -> Self {
        Self {}
    }
}
impl State for ContractState {}
