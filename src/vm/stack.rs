pub type StackPointer = u32;
pub type StackItem = u8;

pub trait Stack {
    fn get_byte(&self, offset: StackPointer) -> StackItem;
}

impl Stack for Vec<StackItem> {
    fn get_byte(&self, offset: StackPointer) -> StackItem {
        return self[self.len() - 1 - offset as usize];
    }
}
