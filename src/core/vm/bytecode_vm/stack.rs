#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackItem {
    Int(i32),
    Bool(bool),
    Byte(u8),
    Bytes([u8; 64]),
}

impl StackItem {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Int(i) => i.to_le_bytes().to_vec(),
            Self::Bool(b) => vec![*b as u8],
            Self::Byte(b) => vec![*b],
            Self::Bytes(b) => b.to_vec(),
        }
    }
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

        data[..N - 1].copy_from_slice(self.data[1..].as_ref());

        self.data = data;

        // Decrease Stack Pointer by 1 or set it to 0 if it is already 0
        self.sp = self.sp.saturating_sub(1);

        val
    }

    pub fn push_front(&mut self, item: StackItem) {
        let mut data = [StackItem::default(); N];

        // Set item to first element of Array
        // Copy all other elements from old array to new array starting from second element
        data[1..].copy_from_slice(self.data[..N - 1].as_ref());
        data[0] = item;

        self.data = data;

        self.sp += 1;
    }
}
