use crate::core::state::{DynState, State};

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

    fn arithmetic_operation<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(i32, i32) -> i32,
    {
        let a = self.stack.pop();
        let b = self.stack.pop();

        if let StackItem::Int(a) = a {
            if let StackItem::Int(b) = b {
                self.stack.push_front(StackItem::Int(f(a, b)));
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("Invalid Stack Items"))
    }

    async fn execute_instruction(
        &mut self,
        state: &DynState,
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
                self.arithmetic_operation(|a, b| a + b)?;
            }
            Instruction::Sub => {
                self.arithmetic_operation(|a, b| a - b)?;
            }
            Instruction::Get => {
                let key = self.stack.pop();
                let val = state.get(&key.to_bytes()).await?;

                match val.len() {
                    1 => {
                        let item = StackItem::Byte(val[0]);
                        self.stack.push_front(item)
                    }
                    4 => {
                        let item =
                            StackItem::Int(i32::from_le_bytes([val[0], val[1], val[2], val[3]]));
                        self.stack.push_front(item);
                    }
                    64 => {
                        let mut bytes = [0u8; 64];
                        bytes.copy_from_slice(&val);
                        let item = StackItem::Bytes(bytes);
                        self.stack.push_front(item);
                    }
                    _ => {}
                }
            }
            Instruction::Mul => {
                self.arithmetic_operation(|a, b| a * b)?;
            }
            Instruction::Div => {
                self.arithmetic_operation(|a, b| a / b)?;
            }
            Instruction::Store => {
                let key = self.stack.pop();
                let val = self.stack.pop();

                state.set(&key.to_bytes(), &val.to_bytes()).await?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl<const N: usize> VM for BytecodeVM<N> {
    async fn execute(&mut self, state: &DynState, code: &[u8]) -> Result<()> {
        self.ip = 0;

        loop {
            // TODO: handle this better (maybe check previous instruction and determine if we are getting a value)
            if let Ok(instr) = Instruction::try_from(code[self.ip]) {
                self.execute_instruction(state, instr, code).await?;
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
    use crate::core::state::mem_state::MemState;

    use super::*;

    #[tokio::test]
    async fn test_vm() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x02, 0xaa];
        let state = Box::new(MemState::new()) as DynState;
        vm.execute(&state, &code).await?;
        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(2));
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_add() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x02, 0xaa, 0x03, 0xaa, 0xad];
        let state = Box::new(MemState::new()) as DynState;
        vm.execute(&state, &code).await?;
        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(5));
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_sub() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x03, 0xaa, 0x02, 0xaa, 0xae];
        let state = Box::new(MemState::new()) as DynState;
        vm.execute(&state, &code).await?;
        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(-1));
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_mul() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x03, 0xaa, 0x02, 0xaa, 0xba];
        let state = Box::new(MemState::new()) as DynState;
        vm.execute(&state, &code).await?;
        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(6));
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_div() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x02, 0xaa, 0x02, 0xaa, 0xbb];
        let state = Box::new(MemState::new()) as DynState;
        vm.execute(&state, &code).await?;
        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(1));
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_store() -> Result<()> {
        let mut vm: BytecodeVM<256> = BytecodeVM::new();
        let code = vec![0x02, 0xaa, 0x04, 0xaa, 0xbc];
        let state = Box::new(MemState::new()) as DynState;
        vm.execute(&state, &code).await?;
        let v = state.get(&[4, 0, 0, 0]).await?;
        assert_eq!(v, vec![2, 0, 0, 0]);

        vm.execute(&state, &[0x04, 0xaa, 0xaf]).await?;

        let res = vm.stack.pop();
        assert_eq!(res, StackItem::Int(2));

        Ok(())
    }
}
