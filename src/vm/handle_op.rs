use crate::vm::stack::Stack;
use crate::vm::stack::{StackPointer, StackItem};
use crate::vm::chunk::ChunkOffset;

pub type OpcodeHandler = fn(&[StackPointer], ChunkOffset, &mut Vec<StackItem>) -> ChunkOffset;

pub fn handle_nop(params: &[StackPointer], offset: ChunkOffset, _stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(params.len(), 0);

    println!("NOP");

    return offset + 1;
}

pub fn handle_return(params: &[StackPointer], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(params.len(), 1);

    println!("RETURN {} ({})", params[0], stack.get_byte(params[0]));

    stack.push(stack.get_byte(params[0]));

    return offset + 1 + params.len() * 4;
}

pub fn handle_add(params: &[StackPointer], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(params.len(), 2);

    println!("ADD {} ({}) {} ({})", params[0], stack.get_byte(params[0]), params[1], stack.get_byte(params[1]));

    stack.push(stack.get_byte(params[0]) + stack.get_byte(params[1]));

    return offset + 1 + params.len() * 4;
}

pub fn handle_constant(params: &[StackPointer], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(params.len(), 1);

    println!("CONSTANT {}", params[0]);

    stack.push(params[0] as StackItem);

    return offset + 1 + params.len() * 4;
}
