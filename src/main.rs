use crate::vm::chunk::Chunk;

mod parser;
mod vm;

fn main() {
    let mut chunk = Vec::<u8>::new();
    chunk.write_op(vm::opcode::OpCode::Nop);
    chunk.write_op(vm::opcode::OpCode::Constant);
    chunk.write_arg(6);
    chunk.write_op(vm::opcode::OpCode::Constant);
    chunk.write_arg(7);
    chunk.write_op(vm::opcode::OpCode::Add);
    chunk.write_arg(0);
    chunk.write_arg(1);

    chunk.execute();
}
