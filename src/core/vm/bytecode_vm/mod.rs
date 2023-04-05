use crate::core::state::State;

mod instruction;
mod stack;

use self::{
    instruction::Instruction,
    stack::{Stack, StackItem},
};

use super::VM;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct BytecodeVM<const N: usize> {
    // Instruction Pointer
    ip: usize,
    stack: Stack<N>,
}

impl<const N: usize> BytecodeVM<N> {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Stack::new(),
        }
    }

    pub fn execute_instruction(
        &mut self,
        state: &dyn State,
        instr: Instruction,
        code: &[u8],
    ) -> Result<()> {
        match instr {
            Instruction::PushBool => {
                let i = self.ip.saturating_sub(1);
                let item = StackItem::Bool(code[i] != 0);
                self.stack.push_front(item);
            }
            Instruction::PushInt => {
                let i = self.ip.saturating_sub(1);
                let item = StackItem::Int(code[i] as i32);
                self.stack.push_front(item);
            }
            Instruction::PushByte => {
                let i = self.ip.saturating_sub(1);
                let item = StackItem::Byte(code[i]);
                self.stack.push_front(item);
            }
            Instruction::Add => {
                let a = self.stack.pop();
                let b = self.stack.pop();

                if let StackItem::Int(a) = a {
                    if let StackItem::Int(b) = b {
                        self.stack.push_front(StackItem::Int(a + b));
                        return Ok(());
                    }
                }
                return Err(anyhow::anyhow!(
                    "Can't run {:?} on {:?} and {:?}",
                    instr,
                    a,
                    b
                ));
            }
            Instruction::Get => {
                let key = self.stack.pop();
                // if let StackItem::Byte(key) = key {
                //     let val = state.get(key);
                //     self.stack.push_front(val);
                // }
            }
            _ => {}
        }

        Ok(())
    }
}

impl<const N: usize> VM for BytecodeVM<N> {
    fn execute(&mut self, state: &dyn State, code: &[u8]) -> Result<()> {
        loop {
            // TODO: handle this better (maybe check previous instruction and determine if we are getting a value)
            if let Ok(instr) = Instruction::try_from(code[self.ip]) {
                self.execute_instruction(state, instr, code)?;
            }

            self.ip += 1;

            if self.ip >= code.len() {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::state::contract_state::ContractState;

    use super::*;

    #[test]
    fn test_vm() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x02, 0xaa, 0x03, 0xaa, 0xad, 0x02, 0xaa, 0xad];
        vm.execute(&ContractState::new(), &code)?;
        println!("{:?}", vm.stack);
        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(7));
        Ok(())
    }
}
