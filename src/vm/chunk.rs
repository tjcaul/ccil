
use crate::vm::opcode::OpCode;

pub trait Chunk {
    fn write_byte(&mut self, byte: u8);
    fn write_op(&mut self, opcode: OpCode);
    fn write_arg(&mut self, arg: u32);
    fn read_arg(&self, offset: usize) -> u32;
    fn execute(&self);
}

impl Chunk for Vec<u8> {
    fn write_byte(&mut self, byte: u8) {
        self.push(byte);
    }

    fn write_op(&mut self, opcode: OpCode) {
        self.write_byte(opcode.byte_value())
    }

    fn write_arg(&mut self, arg: u32) {
        self.write_byte(arg as u8);
        self.write_byte((arg >> 8) as u8);
        self.write_byte((arg >> 16) as u8);
        self.write_byte((arg >> 24) as u8);
    }

    fn read_arg(&self, offset: usize) -> u32 {
        self[offset] as u32
        | (self[offset+1] as u32) << 8
        | (self[offset+2] as u32) << 16
        | (self[offset+3] as u32) << 24
    }

    fn execute(&self) {
        let mut offset = 0;
        let mut stack = Vec::<u8>::new();

        while offset < self.len() {
            // Get opcode at current pos (guaranteed to be opcode by invariant)
            print!("{} ", offset);
            let chunk_code = OpCode::from_byte(self[offset]);
            
            // get handler, and number of args
            let (handler, num_args) = chunk_code.get_handler();
            let mut args = Vec::<usize>::new();
            for i in 0..num_args {
                args.push(self.read_arg(offset + 1 + 4*i) as usize);
            }

            // Run handler for op, we get next part of function and possible value to push to stack
            let new_offset = handler(&args, offset, &mut stack);
            offset = new_offset;
            println!("\t{:?}", stack);
        }
    }
}
