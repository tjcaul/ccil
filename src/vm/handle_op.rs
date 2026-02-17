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

use std::io::Write;

use rustc_hash::FxHashMap;

use crate::dprintln;
use crate::vm::VirtualMachine;
use crate::vm::stack::{VecStack, Stack, StackPointer, StackItem, Shift};
use crate::vm::chunk::ChunkOffset;
use crate::vm::opcode::Argument;
use crate::vm::variable_value::VariableValue;
use crate::constants::{fileno_const, type_id_const};

pub type OpcodeHandler = fn(&mut VirtualMachine, &[Argument], ChunkOffset) -> Result<Option<ChunkOffset>, String>;

const POP_ERROR_STR: &str = "Cannot pop from empty stack";

fn compute_opcode_size(num_args: usize) -> ChunkOffset {
    1 + num_args * (Argument::BITS as usize) / (u8::BITS as usize)
}

pub fn handle_nop(_vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    dprintln!("NOP");

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_constant(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let constant = args[0] as StackItem;
    vm.stack.push(constant);
    dprintln!("CONST {}", constant);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_pop(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    dprintln!("POP ({})", val);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_drop(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let count = args[0] as usize;
    for _ in 0..count {
        vm.stack.pop().ok_or(POP_ERROR_STR)?;
    }
    dprintln!("DROP {}", count);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_copy(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as StackPointer;
    let data = vm.stack.get(address);
    vm.stack.push(data);
    dprintln!("COPY {} ({})", address, data);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_store(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 2);

    let variable_id = args[0];
    let type_id = args[1];
    let data = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    
    let value = match type_id {
        type_id_const::STRING => VariableValue::StringLiteral(data as usize),
        type_id_const::NUMBER => VariableValue::Number(data),
        type_id_const::FLOAT => todo!(),
        type_id_const::NULL => VariableValue::Null,
        type_id_const::BOOLEAN => VariableValue::Boolean(data != 0),
        type_id_const::UNKNOWN => todo!(), // add runtime type inference here
        _ => todo!()
    };
    vm.variables.insert(variable_id, value);

    dprintln!("STORE {} {} ({})", variable_id, type_id, data);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_load(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let variable_id = &args[0];
    let value = match vm.variables.get(variable_id) {
        Some(val) => val,
        None => panic!("Attempted to access valueless variable")
    };

    match value {
        VariableValue::StringLiteral(val) => vm.stack.push(*val as i32),
        VariableValue::Number(val) => vm.stack.push(*val),
        VariableValue::Float(_) => todo!(),
        VariableValue::Null => vm.stack.push(0),
        VariableValue::Boolean(val) => {
            if *val {
                vm.stack.push(1)
            } else {
                vm.stack.push(0)
            }
        },
    }

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_swap(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    vm.stack.push(b);
    vm.stack.push(a);
    dprintln!("SWAP {} {} -> {} {}", a, b, b, a);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_rot(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let count = args[0] as StackPointer;

    let item_moving_down = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    vm.stack.insert(count, item_moving_down);
    dprintln!("ROT {}", count);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_neg(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let negative = -val;
    vm.stack.push(negative);
    dprintln!("NEG {} -> {}", val, negative);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_add(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let sum = a + b;
    vm.stack.push(sum);
    dprintln!("ADD {} {} -> {}", a, b, sum);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_sub(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let difference = a - b;
    vm.stack.push(difference);
    dprintln!("SUB {} {} -> {}", a, b, difference);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_mul(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let product = a - b;
    vm.stack.push(product);
    dprintln!("MUL {} {} -> {}", a, b, product);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_div(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let divisor = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let dividend = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let quotient = dividend / divisor;
    vm.stack.push(quotient);
    dprintln!("DIV {} {} -> {}", dividend, divisor, quotient);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_mod(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let divisor = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let dividend = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let remainder = dividend % divisor;
    vm.stack.push(remainder);
    dprintln!("MOD {} {} -> {}", dividend, divisor, remainder);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_bnot(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_not = !val;
    vm.stack.push(bitwise_not);
    dprintln!("BNOT {} -> {}", val, bitwise_not);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_bor(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_or = a | b;
    vm.stack.push(bitwise_or);
    dprintln!("BOR {} {} -> {}", a, b, bitwise_or);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_band(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_and = a & b;
    vm.stack.push(bitwise_and);
    dprintln!("BOR {} {} -> {}", a, b, bitwise_and);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_bxor(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let bitwise_xor = a ^ b;
    vm.stack.push(bitwise_xor);
    dprintln!("BXOR {} {} -> {}", a, b, bitwise_xor);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_not(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let val = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_not = !val;
    vm.stack.push(boolean_not as StackItem);
    dprintln!("NOT {} -> {}", val, boolean_not);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_or(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_or = a || b;
    vm.stack.push(boolean_or as StackItem);
    dprintln!("OR {} {} -> {}", a, b, boolean_or);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_and(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_and = a && b;
    vm.stack.push(boolean_and as StackItem);
    dprintln!("AND {} {} -> {}", a, b, boolean_and);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_xor(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let b = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let a = vm.stack.pop().ok_or(POP_ERROR_STR)? != 0;
    let boolean_xor = (a && !b) || (!a && b);
    vm.stack.push(boolean_xor as StackItem);
    dprintln!("XOR {} {} -> {}", a, b, boolean_xor);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_shl(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let shift_amount = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let value = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let shifted = shift_amount << value;
    vm.stack.push(shifted as StackItem);
    dprintln!("SHL {} {} -> {}", value, shift_amount, shifted);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_shrl(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let shift_amount = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let value = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let shifted = value.logical_shift(shift_amount);
    vm.stack.push(shifted as StackItem);
    dprintln!("SHRL {} {} -> {}", value, shift_amount, shifted);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_shra(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    let shift_amount = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let value = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    let shifted = value.logical_shift(shift_amount);
    vm.stack.push(shifted as StackItem);
    dprintln!("SHRA {} {} -> {}", value, shift_amount, shifted);

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_jump(_vm: &mut VirtualMachine, args: &[Argument], _offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    dprintln!("JUMP {}", address);

    Ok(Some(address))
}

pub fn handle_ifz(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    let condition = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    dprintln!("IFZ {} ({})", address, condition);

    if condition == 0 {
        Ok(Some(address))
    } else {
        Ok(Some(offset + compute_opcode_size(args.len())))
    }

}

pub fn handle_ifnz(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let address = args[0] as ChunkOffset;
    let condition = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    dprintln!("IFNZ {} ({})", address, condition);

    if condition != 0 {
        Ok(Some(address))
    } else {
        Ok(Some(offset + compute_opcode_size(args.len())))
    }

}

pub fn handle_call(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let call_address = args[0] as ChunkOffset;
    let return_address = offset + compute_opcode_size(args.len());
    vm.stack.push(return_address as StackItem);
    dprintln!("CALL {}", call_address);

    Ok(Some(call_address))
}

pub fn handle_return(vm: &mut VirtualMachine, args: &[Argument], _offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let discard_count = args[0] as usize;
    for _ in 0..discard_count {
        vm.stack.pop().ok_or(POP_ERROR_STR)?;
    }

    let return_address = vm.stack.pop().ok_or(POP_ERROR_STR)?;
    assert!(return_address >= 0, "Return address must be non-negative");

    dprintln!("RETURN {} -> ({})", discard_count, return_address);

    Ok(Some(return_address as ChunkOffset))
}

pub fn handle_exit(args: &[Argument], _offset: ChunkOffset, _stack: &mut VecStack, _variables: &mut FxHashMap<i32, VariableValue>) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 0);

    dprintln!("EXIT");

    Ok(None)
}

pub fn handle_write(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let fileno = args[0];
    let value = vm.stack.pop().ok_or(POP_ERROR_STR)?;

    dprintln!("WRITE {}", fileno);

    match fileno {
        fileno_const::STDIN => panic!("Cannot write to STDIN"),
        fileno_const::STDOUT => println!("{}", value),
        fileno_const::STDERR => eprintln!("{}", value),
        other_value => {
            let mut file = &vm.opened_files[other_value as usize - 2];
            let _ = write!(file, "{}", value);
        }
    }

    Ok(Some(offset + compute_opcode_size(args.len())))
}

pub fn handle_writes(vm: &mut VirtualMachine, args: &[Argument], offset: ChunkOffset) -> Result<Option<ChunkOffset>, String> {
    assert_eq!(args.len(), 1);

    let fileno = args[0];
    let string_pointer = vm.stack.pop().ok_or(POP_ERROR_STR)? as usize;
    let write_string = vm.get_string(string_pointer);

    dprintln!("WRITES {}", fileno);
    
    match fileno {
        fileno_const::STDIN => panic!("Cannot write to STDIN"),
        fileno_const::STDOUT => println!("{}", write_string),
        fileno_const::STDERR => eprintln!("{}", write_string),
        other_value => {
            let mut file = &vm.opened_files[other_value as usize - 2];
            let _ = write!(file, "{}", write_string);
        }
    }

    Ok(Some(offset + compute_opcode_size(args.len())))
}
