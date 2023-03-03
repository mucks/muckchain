use crate::core::state::State;

use super::VM;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum StackItem {
    Int(i64),
    Bool(bool),
    Bytes([u8; 64]),
}

impl Default for StackItem {
    fn default() -> Self {
        Self::Int(0)
    }
}

#[derive(Debug, Clone)]
pub struct Stack<const N: usize> {
    data: [StackItem; N],
    sp: usize,
}

impl<const N: usize> Stack<N> {
    pub fn new() -> Self {
        Self {
            data: [StackItem::default(); N],
            sp: 0,
        }
    }

    pub fn pop(&mut self) -> StackItem {
        let val = self.data[0];

        let mut data: [StackItem; N] = self.data;
        // Copy all other elements from old array to new array starting from first element
        data.copy_from_slice(self.data[1..].as_ref());

        self.data = data;

        // Decrease Stack Pointer by 1 or set it to 0 if it is already 0
        self.sp = self.sp.saturating_sub(1);

        val
    }

    pub fn push_front(&mut self, item: StackItem) {
        let mut data = [StackItem::default(); N];

        // Set item to first element of Array
        data[0] = item;
        // Copy all other elements from old array to new array starting from second element
        data[1..].copy_from_slice(self.data[..N - 1].as_ref());

        self.data = data;

        self.sp += 1;
    }
}

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
}

impl<const N: usize> VM for BytecodeVM<N> {
    fn execute(&self, state: &dyn State, code: &[u8]) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
