/*
vm/handle_op.rs: Defines handlers for OpCode behaviour
Copyright (C) 2025-26 The CCIL Developers

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::vm::stack::{VecStack, Stack, StackPointer, StackItem, Shift};
use crate::vm::chunk::ChunkOffset;
use crate::vm::opcode::Argument;

pub type OpcodeHandler = fn(&[Argument], ChunkOffset, &mut VecStack) -> Result<Option<ChunkOffset>, String>;

const POP_ERROR_STR: &str = "Cannot pop from empty stack";

fn compute_opcode_size(num_args: usize) -> ChunkOffset {
    1 + num_args * (Argument::BITS as usize) / (u8::BITS as usize)
}

pub fn handle_nop(args: &[Argument], offset: ChunkOffset, _stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    println!("NOP");

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_constant(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let constant = args[0] as StackItem;
    stack.push(constant);
    println!("CONST {}", constant);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_pop(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = stack.pop().ok_or(POP_ERROR_STR)?;
    println!("POP ({})", val);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_drop(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let count = args[0] as usize;
    for _ in 0..count {
        stack.pop().ok_or(POP_ERROR_STR)?;
    }
    println!("DROP {}", count);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_copy(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = stack.get(address);
    stack.push(data);
    println!("COPY {} ({})", address, data);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_store(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = stack.pop().ok_or(POP_ERROR_STR)?;
    stack.set(address, data);
    println!("STORE {} ({})", address, data);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_swap(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    stack.push(b);
    stack.push(a);
    println!("SWAP {} {} -> {} {}", a, b, b, a);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_rot(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let count = args[0] as StackPointer;

    let item_moving_down = stack.pop().ok_or(POP_ERROR_STR)?;
    stack.insert(count, item_moving_down);
    println!("ROT {}", count);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_neg(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = stack.pop().ok_or(POP_ERROR_STR)?;
    let negative = -val;
    stack.push(negative);
    println!("NEG {} -> {}", val, negative);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_add(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    let sum = a + b;
    stack.push(sum);
    println!("ADD {} {} -> {}", a, b, sum);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_sub(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    let difference = a - b;
    stack.push(difference);
    println!("SUB {} {} -> {}", a, b, difference);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_mul(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    let product = a - b;
    stack.push(product);
    println!("MUL {} {} -> {}", a, b, product);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_div(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let divisor = stack.pop().ok_or(POP_ERROR_STR)?;
    let dividend = stack.pop().ok_or(POP_ERROR_STR)?;
    let quotient = dividend / divisor;
    stack.push(quotient);
    println!("DIV {} {} -> {}", dividend, divisor, quotient);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_mod(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let divisor = stack.pop().ok_or(POP_ERROR_STR)?;
    let dividend = stack.pop().ok_or(POP_ERROR_STR)?;
    let remainder = dividend % divisor;
    stack.push(remainder);
    println!("MOD {} {} -> {}", dividend, divisor, remainder);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_bnot(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_not = !val;
    stack.push(bitwise_not);
    println!("BNOT {} -> {}", val, bitwise_not);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_bor(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_or = a | b;
    stack.push(bitwise_or);
    println!("BOR {} {} -> {}", a, b, bitwise_or);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_band(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_and = a & b;
    stack.push(bitwise_and);
    println!("BOR {} {} -> {}", a, b, bitwise_and);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_bxor(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)?;
    let a = stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_xor = a ^ b;
    stack.push(bitwise_xor);
    println!("BXOR {} {} -> {}", a, b, bitwise_xor);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_not(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_not = !val;
    stack.push(boolean_not as StackItem);
    println!("NOT {} -> {}", val, boolean_not);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_or(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let a = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_or = a || b;
    stack.push(boolean_or as StackItem);
    println!("OR {} {} -> {}", a, b, boolean_or);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_and(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let a = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_and = a && b;
    stack.push(boolean_and as StackItem);
    println!("AND {} {} -> {}", a, b, boolean_and);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_xor(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let a = stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_xor = (a && !b) || (!a && b);
    stack.push(boolean_xor as StackItem);
    println!("XOR {} {} -> {}", a, b, boolean_xor);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_shl(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let shift_amount = stack.pop().ok_or(POP_ERROR_STR)?;
    let value = stack.pop().ok_or(POP_ERROR_STR)?;
    let shifted = shift_amount << value;
    stack.push(shifted as StackItem);
    println!("SHL {} {} -> {}", value, shift_amount, shifted);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_shrl(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let shift_amount = stack.pop().ok_or(POP_ERROR_STR)?;
    let value = stack.pop().ok_or(POP_ERROR_STR)?;
    let shifted = value.logical_shift(shift_amount);
    stack.push(shifted as StackItem);
    println!("SHRL {} {} -> {}", value, shift_amount, shifted);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_shra(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let shift_amount = stack.pop().ok_or(POP_ERROR_STR)?;
    let value = stack.pop().ok_or(POP_ERROR_STR)?;
    let shifted = value.logical_shift(shift_amount);
    stack.push(shifted as StackItem);
    println!("SHRA {} {} -> {}", value, shift_amount, shifted);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_jump(args: &[Argument], _offset: ChunkOffset, _stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    println!("JUMP {}", address);

    Ok(Some(address))
}

pub fn handle_ifz(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    let condition = stack.pop().ok_or(POP_ERROR_STR)?;
    println!("IFZ {} ({})", address, condition);

    if condition == 0 {
        Ok(Some(address))
    } else {
        Ok(Some(offset + compute_opcode_size(args.len())))
    }

}

pub fn handle_ifnz(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    let condition = stack.pop().ok_or(POP_ERROR_STR)?;
    println!("IFNZ {} ({})", address, condition);

    if condition != 0 {
        Ok(Some(address))
    } else {
        Ok(Some(offset + compute_opcode_size(args.len())))
    }

}

pub fn handle_call(args: &[Argument], offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let call_address = args[0] as ChunkOffset;
    let return_address = offset + compute_opcode_size(args.len());
    stack.push(return_address as StackItem);
    println!("CALL {}", call_address);

    Ok(Some(call_address))
}

pub fn handle_return(args: &[Argument], _offset: ChunkOffset, stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let discard_count = args[0] as usize;
    for _ in 0..discard_count {
        stack.pop().ok_or(POP_ERROR_STR)?;
    }

    let return_address = stack.pop().ok_or(POP_ERROR_STR)?;
    assert!(return_address >= 0, "Return address must be non-negative");

    println!("RETURN {} -> ({})", discard_count, return_address);

    Ok(Some(return_address as ChunkOffset))
}

pub fn handle_exit(args: &[Argument], _offset: ChunkOffset, _stack: &mut VecStack) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    println!("EXIT");

    Ok(None)
}
