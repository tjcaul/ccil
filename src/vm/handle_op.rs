use crate::vm::stack::Stack;
use crate::vm::stack::{StackPointer, StackItem};
use crate::vm::chunk::ChunkOffset;
use crate::vm::opcode::Argument;

pub type OpcodeHandler = fn(&[Argument], ChunkOffset, &mut Vec<StackItem>) -> ChunkOffset;

const POP_ERROR_STR: &str = "Popped from empty stack";

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

pub fn handle_pop(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let val = stack.pop().expect(POP_ERROR_STR);
    println!("POP ({})", val);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_copy(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = stack.get(address);
    stack.push(data);
    println!("PUSH {} ({})", address, data);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_swap(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    stack.push(b);
    stack.push(a);
    println!("SWAP {} {} -> {} {}", a, b, b, a);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_neg(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let val = stack.pop().expect(POP_ERROR_STR);
    let negative = -val;
    stack.push(negative);
    println!("NEG {} -> {}", val, negative);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_add(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    let sum = a + b;
    stack.push(sum);
    println!("ADD {} {} -> {}", a, b, sum);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_sub(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    let difference = a - b;
    stack.push(difference);
    println!("SUB {} {} -> {}", a, b, difference);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_mul(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    let product = a - b;
    stack.push(product);
    println!("MUL {} {} -> {}", a, b, product);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_div(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let divisor = stack.pop().expect(POP_ERROR_STR);
    let dividend = stack.pop().expect(POP_ERROR_STR);
    let quotient = dividend / divisor;
    stack.push(quotient);
    println!("DIV {} {} -> {}", dividend, divisor, quotient);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_mod(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let divisor = stack.pop().expect(POP_ERROR_STR);
    let dividend = stack.pop().expect(POP_ERROR_STR);
    let remainder = dividend % divisor;
    stack.push(remainder);
    println!("MOD {} {} -> {}", dividend, divisor, remainder);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_bnot(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let val = stack.pop().expect(POP_ERROR_STR);
    let bitwise_not = !val;
    stack.push(bitwise_not);
    println!("BNOT {} -> {}", val, bitwise_not);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_bor(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    let bitwise_or = a | b;
    stack.push(bitwise_or);
    println!("BOR {} {} -> {}", a, b, bitwise_or);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_band(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    let bitwise_and = a & b;
    stack.push(bitwise_and);
    println!("BOR {} {} -> {}", a, b, bitwise_and);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_bxor(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR);
    let a = stack.pop().expect(POP_ERROR_STR);
    let bitwise_xor = a ^ b;
    stack.push(bitwise_xor);
    println!("BXOR {} {} -> {}", a, b, bitwise_xor);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_not(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let val = stack.pop().expect(POP_ERROR_STR) != 0;
    let boolean_not = !val;
    stack.push(boolean_not as StackItem);
    println!("NOT {} -> {}", val, boolean_not);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_or(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR) != 0;
    let a = stack.pop().expect(POP_ERROR_STR) != 0;
    let boolean_or = a || b;
    stack.push(boolean_or as StackItem);
    println!("OR {} {} -> {}", a, b, boolean_or);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_and(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR) != 0;
    let a = stack.pop().expect(POP_ERROR_STR) != 0;
    let boolean_and = a && b;
    stack.push(boolean_and as StackItem);
    println!("AND {} {} -> {}", a, b, boolean_and);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_xor(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let b = stack.pop().expect(POP_ERROR_STR) != 0;
    let a = stack.pop().expect(POP_ERROR_STR) != 0;
    let boolean_xor = (a && !b) || (!a && b);
    stack.push(boolean_xor as StackItem);
    println!("XOR {} {} -> {}", a, b, boolean_xor);

    return offset + compute_opcode_size(args.len());
}
