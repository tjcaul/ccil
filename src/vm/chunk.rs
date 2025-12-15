use crate::vm::opcode::{OpCode, OpCodeLookup};
use crate::vm::stack::{StackPointer, StackItem};

pub type ChunkOffset = usize;

#[allow(unused)]
pub trait Chunk {
    fn from_file(path: &str) -> Self;
    fn to_file(&self, path: &str);
    fn write_byte(&mut self, byte: u8);
    fn write_op(&mut self, opcode: &OpCode);
    fn write_arg(&mut self, arg: StackPointer);
    fn read_arg(&self, offset: ChunkOffset) -> StackPointer;
    fn execute(&self, lookup: &OpCodeLookup);
}

impl Chunk for Vec<u8> {
    fn from_file(path: &str) -> Self {
        let result = std::fs::read(path);
        match result {
            Ok(vec) => vec,
            Err(_) => panic!("Failed to read chunk file")
        }
    }

    fn to_file(&self, path: &str) {
        let result = std::fs::write(path, self);
        match result {
            Ok(_) => {},
            Err(_) => panic!("Failed to write chunk file")
        }
    }

    fn write_byte(&mut self, byte: u8) {
        self.push(byte);
    }

    fn write_op(&mut self, opcode: &OpCode) {
        self.write_byte(opcode.byte)
    }

    fn write_arg(&mut self, arg: StackPointer) {
        self.write_byte(arg as u8);
        self.write_byte((arg >> 8) as u8);
        self.write_byte((arg >> 16) as u8);
        self.write_byte((arg >> 24) as u8);
    }

    fn read_arg(&self, offset: ChunkOffset) -> StackPointer {
        self[offset] as StackPointer
        | (self[offset+1] as StackPointer) << 8
        | (self[offset+2] as StackPointer) << 16
        | (self[offset+3] as StackPointer) << 24
    }

    fn execute(&self, lookup: &OpCodeLookup) {
        let mut offset = 0;
        let mut stack = Vec::<StackItem>::new();

        while offset < self.len() {
            // Get opcode at current pos (guaranteed to be opcode by invariant)
            print!("{} ", offset);
            let chunk_code = match lookup.from_byte(self[offset]) {
                Some(opcode) => opcode,
                None => panic!("Unknown opcode with value {:x}", self[offset])
            };
            
            let mut args = Vec::<StackPointer>::new();
            for i in 0..chunk_code.num_params {
                args.push(self.read_arg(offset + 1 + 4*i) as StackPointer);
            }

            // Run handler for op, we get next offset
            offset = (chunk_code.handler)(&args, offset, &mut stack);
            println!("\t{:?}", stack);
        }
    }
}
