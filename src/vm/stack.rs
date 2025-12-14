pub trait Stack {
    fn get_byte(&self, offset: usize) -> u8;
}

impl Stack for Vec<u8> {
    fn get_byte(&self, offset: usize) -> u8 {
        return self[self.len() - 1 - offset];
    }
}