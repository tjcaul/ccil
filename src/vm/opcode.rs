use crate::vm::handle_op;

pub enum OpCode {
    Nop,
    Return,
    Add,
    Constant
}

impl OpCode {
    pub fn byte_value(&self) -> u8 {
        match *self {
            OpCode::Nop => 0,
            OpCode::Return => 1,
            OpCode::Add => 2,
            OpCode::Constant => 3
        }
    }
    
    pub fn get_handler(&self) -> (impl Fn(&[usize], usize, &mut Vec<u8>) -> usize, usize) {
        match *self {
            OpCode::Nop => (handle_op::handle_nop as fn(&[usize], usize, &mut Vec<u8>) -> usize, 0),
            OpCode::Return => (handle_op::handle_return as fn(&[usize], usize, &mut Vec<u8>) -> usize, 1),
            OpCode::Add => (handle_op::handle_add as fn(&[usize], usize, &mut Vec<u8>) -> usize, 2),
            OpCode::Constant => (handle_op::handle_constant as fn(&[usize], usize, &mut Vec<u8>) -> usize, 1),
        }
    }

    pub const fn from_byte(byte: u8) -> Self {
        match byte {
            0 => OpCode::Nop,
            1 => OpCode::Return,
            2 => OpCode::Add,
            3 => OpCode::Constant,
            4_u8..=u8::MAX => todo!()
        }
    }
}
