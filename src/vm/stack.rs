pub type StackPointer = i32;
pub type StackItem = i8;

pub trait Shift<T> {
    fn logical_shift(self, shift_amount: T) -> Self;
    fn arithmetic_shift(self, shift_amount: T) -> Self;
}

impl Shift<StackItem> for i8 {
    fn logical_shift(self, shift_amount: StackItem) -> Self {
        assert!(shift_amount >= 0);

        if shift_amount == 0 {
            return self;
        }

        // We know shift_amount >= 1, so MSB will be 0 after shift, so
        // u8->i8 conversion is safe.
        return ((self as u8) >> shift_amount).try_into().unwrap();
    }

    fn arithmetic_shift(self, shift_amount: StackItem) -> Self {
        return (self as i8) >> (shift_amount as usize) as Self;
    }
}

pub trait Stack {
    fn get(&self, offset: StackPointer) -> StackItem;
    fn insert(&mut self, offset: StackPointer, item: StackItem);
    fn set(&mut self, offset: StackPointer, item: StackItem);
}

impl Stack for Vec<StackItem> {
    fn get(&self, offset: StackPointer) -> StackItem {
        let index = self.len() - 1 - offset as usize;
        return self[index];
    }
    
    fn insert(&mut self, offset: StackPointer, item: StackItem) {
        Vec::<StackItem>::insert(self, self.len() - offset as usize, item);
    }

    fn set(&mut self, offset: StackPointer, item: StackItem) {
        let index = self.len() - 1 - offset as usize;
        self[index] = item;
    }
}
