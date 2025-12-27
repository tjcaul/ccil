use crate::vm::stack::Stack;
use crate::vm::stack::{StackPointer, StackItem, Shift};
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

    let constant = args[0] as StackItem;
    stack.push(constant);
    println!("CONST {}", constant);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_pop(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let val = stack.pop().expect(POP_ERROR_STR);
    println!("POP ({})", val);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_drop(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let count = args[0] as usize;
    for _ in 0..count {
        stack.pop().expect(POP_ERROR_STR);
    }
    println!("DROP {}", count);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_copy(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = stack.get(address);
    stack.push(data);
    println!("COPY {} ({})", address, data);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_store(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = stack.pop().expect(POP_ERROR_STR);
    stack.set(address, data);
    println!("STORE {} ({})", address, data);

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

pub fn handle_rot(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let count = args[0] as StackPointer;

    let item_moving_down = stack.pop().expect(POP_ERROR_STR);
    Stack::insert(stack, count, item_moving_down);
    println!("ROT {}", count);

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

pub fn handle_shl(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let shift_amount = stack.pop().expect(POP_ERROR_STR);
    let value = stack.pop().expect(POP_ERROR_STR);
    let shifted = shift_amount << value;
    stack.push(shifted as StackItem);
    println!("SHL {} {} -> {}", value, shift_amount, shifted);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_shrl(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let shift_amount = stack.pop().expect(POP_ERROR_STR);
    let value = stack.pop().expect(POP_ERROR_STR);
    let shifted = value.logical_shift(shift_amount);
    stack.push(shifted as StackItem);
    println!("SHRL {} {} -> {}", value, shift_amount, shifted);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_shra(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 0);

    let shift_amount = stack.pop().expect(POP_ERROR_STR);
    let value = stack.pop().expect(POP_ERROR_STR);
    let shifted = value.logical_shift(shift_amount);
    stack.push(shifted as StackItem);
    println!("SHRA {} {} -> {}", value, shift_amount, shifted);

    return offset + compute_opcode_size(args.len());
}

pub fn handle_jump(args: &[Argument], _offset: ChunkOffset, _stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    println!("JUMP {}", address);

    return address;
}

pub fn handle_ifz(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    let condition = stack.pop().expect(POP_ERROR_STR);
    println!("IFZ {} ({})", address, condition);

    if condition == 0 {
        return address;
    } else {
        return offset + compute_opcode_size(args.len());
    }

}

pub fn handle_ifnz(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    let condition = stack.pop().expect(POP_ERROR_STR);
    println!("IFNZ {} ({})", address, condition);

    if condition != 0 {
        return address;
    } else {
        return offset + compute_opcode_size(args.len());
    }

}

pub fn handle_call(args: &[Argument], offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let call_address = args[0] as ChunkOffset;
    let return_address = offset + compute_opcode_size(args.len());
    stack.push(return_address as StackItem);
    println!("CALL {}", call_address);

    return call_address;
}

pub fn handle_return(args: &[Argument], _offset: ChunkOffset, stack: &mut Vec<StackItem>) -> ChunkOffset {
    assert_eq!(args.len(), 1);

    let discard_count = args[0] as usize;
    for _ in 0..discard_count {
        stack.pop().expect(POP_ERROR_STR);
    }

    let return_address = stack.pop().expect(POP_ERROR_STR);
    assert!(return_address >= 0, "Return address must be non-negative");

    println!("RETURN {} -> ({})", discard_count, return_address);

    return return_address as usize;
}
