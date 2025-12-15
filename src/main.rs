use crate::vm::{chunk::Chunk, opcode::OpCodeLookup};

mod parser;
mod vm;

fn main() {
    let chunk: Vec<u8> = Chunk::from_file("./bytecode/test.ccilb");
    let opcode_lookup = OpCodeLookup::new();
    /*
    let mut chunk = Vec::<u8>::new();
    chunk.write_op(vm::opcode::OpCode::Nop);
    chunk.write_op(vm::opcode::OpCode::Constant);
    chunk.write_arg(6);
    chunk.write_op(vm::opcode::OpCode::Constant);
    chunk.write_arg(7);
    chunk.write_op(vm::opcode::OpCode::Add);
    chunk.write_arg(0);
    chunk.write_arg(1);
    */

    chunk.execute(&opcode_lookup);
}
