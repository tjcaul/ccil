use crate::vm::stack::Stack;
use crate::vm::stack::{StackPointer, StackItem};
use crate::vm::chunk::ChunkOffset;
use crate::vm::opcode::Argument;

pub type OpcodeHandler = fn(&[Argument], ChunkOffset, &mut Vec<StackItem>) -> ChunkOffset;

fn compute_opcode_size(num_args: usize) -> ChunkOffset {
    1 + num_args * (Argument::BITS as usize) / (u8::BITS as usize)
}

pub fn handle_nop(args: &[Argument], offset: ChunkOffset, _stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    println!("NOP");

    return offset + compute_opcode_size(args.len());
}

pub fn handle_constant(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let data = args[0] as StackItem;
    stack.push(data);
    println!("CONSTANT {}", data);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_push(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = stack.get(address);
    stack.push(data);
    println!("PUSH {} ({})", address, data);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_pop(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let val = stack.pop().expect("Popped from empty stack");
    println!("POP -> {}", val);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_add(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect("Popped from empty stack");
    let a = stack.pop().expect("Popped from empty stack");
    let sum = a + b;
    stack.push(sum);
    println!("ADD {} {} -> {}", a, b, sum);

    return offset + 1 + args.len();
}
