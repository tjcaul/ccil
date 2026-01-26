/*
vm/opcode.rs: Defines opcodes and their lookup tables
Copyright (C) 2025-26 Tyson Caul and Ray Chen

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

use std::collections::BTreeMap;
use crate::vm::handle_op;
use crate::vm::handle_op::OpcodeHandler;

pub type Argument = i32;

#[allow(dead_code)]
pub struct OpCode<'a> {
    pub symbol: &'a str,
    pub byte: u8,
    pub handler: OpcodeHandler,
    pub num_params: usize
}

pub struct OpCodeLookup<'a> {
    symbol_lookup: BTreeMap<&'a str, &'a OpCode<'a>>,
    byte_lookup: Vec<Option<&'a OpCode<'a>>>
}

#[allow(dead_code)]
impl<'a> OpCodeLookup<'a> {
    pub fn new() -> Self {
        // Instantiate lookup tables
        let mut symbol_lookup = BTreeMap::<&str, &OpCode>::new();
        let mut byte_lookup = Vec::<Option<&OpCode>>::new();
        byte_lookup.resize(u8::MAX as usize - u8::MIN as usize + 1, None);

        // Add opcodes to tables
        for i in 0..OPCODES.len() {
            let opcode = &OPCODES[i];
            match &byte_lookup[opcode.byte as usize] {
                &Some(opcode2) => {
                    panic!(
                        "Opcodes {} and {} both have byte 0x{:02x}",
                        opcode.symbol,
                        opcode2.symbol,
                        opcode.byte
                    );
                },
                &None => {
                    byte_lookup[opcode.byte as usize] = Some(opcode);
                }
            }
            match symbol_lookup.get(opcode.symbol) {
                Some(&opcode2) => {
                    panic!(
                        "Opcodes with bytes 0x{:02x} and 0x{:02x} both have symbol {}",
                        opcode.byte,
                        opcode2.byte,
                        opcode.symbol
                    );
                }
                None => {
                    symbol_lookup.insert(opcode.symbol, opcode);
                }
            }
        }

        return Self {symbol_lookup, byte_lookup};
    }

    pub fn from_symbol(&'a self, symbol: &'a str) -> Option<&'a OpCode<'a>> {
        match self.symbol_lookup.get(&symbol.to_uppercase()[..]) {
            Some(&opcode) => Some(opcode),
            None => None
        }
    }

    pub fn from_byte(&'a self, byte: u8) -> Option<&'a OpCode<'a>> {
        return self.byte_lookup[byte as usize];
    }
}

const OPCODES: &[OpCode] = &[
    OpCode {
        symbol: "NOP", byte: 0x00,
        handler: handle_op::handle_nop, num_params: 0
    },
    OpCode {
        symbol: "CONST", byte: 0x01,
        handler: handle_op::handle_constant, num_params: 1
    },
    OpCode {
        symbol: "POP", byte: 0x02,
        handler: handle_op::handle_pop, num_params: 0
    },
    OpCode {
        symbol: "DROP", byte: 0x03,
        handler: handle_op::handle_drop, num_params: 1
    },
    OpCode {
        symbol: "COPY", byte: 0x04,
        handler: handle_op::handle_copy, num_params: 1
    },
    OpCode {
        symbol: "STORE", byte: 0x05,
        handler: handle_op::handle_store, num_params: 1
    },
    OpCode {
        symbol: "SWAP", byte: 0x06,
        handler: handle_op::handle_swap, num_params: 0
    },
    OpCode {
        symbol: "ROT", byte: 0x07,
        handler: handle_op::handle_rot, num_params: 1
    },
    OpCode {
        symbol: "NEG", byte: 0x10,
        handler: handle_op::handle_neg, num_params: 0
    },
    OpCode {
        symbol: "ADD", byte: 0x11,
        handler: handle_op::handle_add, num_params: 0
    },
    OpCode {
        symbol: "SUB", byte: 0x12,
        handler: handle_op::handle_sub, num_params: 0
    },
    OpCode {
        symbol: "MUL", byte: 0x13,
        handler: handle_op::handle_mul, num_params: 0
    },
    OpCode {
        symbol: "DIV", byte: 0x14,
        handler: handle_op::handle_div, num_params: 0
    },
    OpCode {
        symbol: "MOD", byte: 0x15,
        handler: handle_op::handle_mod, num_params: 0
    },
    OpCode {
        symbol: "BNOT", byte: 0x20,
        handler: handle_op::handle_bnot, num_params: 0
    },
    OpCode {
        symbol: "BOR", byte: 0x21,
        handler: handle_op::handle_bor, num_params: 0
    },
    OpCode {
        symbol: "BAND", byte: 0x22,
        handler: handle_op::handle_band, num_params: 0
    },
    OpCode {
        symbol: "BXOR", byte: 0x23,
        handler: handle_op::handle_bxor, num_params: 0
    },
    OpCode {
        symbol: "NOT", byte: 0x24,
        handler: handle_op::handle_not, num_params: 0
    },
    OpCode {
        symbol: "OR", byte: 0x25,
        handler: handle_op::handle_or, num_params: 0
    },
    OpCode {
        symbol: "AND", byte: 0x26,
        handler: handle_op::handle_and, num_params: 0
    },
    OpCode {
        symbol: "XOR", byte: 0x27,
        handler: handle_op::handle_xor, num_params: 0
    },
    OpCode {
        symbol: "SHL", byte: 0x28,
        handler: handle_op::handle_shl, num_params: 0
    },
    OpCode {
        symbol: "SHRL", byte: 0x29,
        handler: handle_op::handle_shrl, num_params: 0
    },
    OpCode {
        symbol: "SHRA", byte: 0x2a,
        handler: handle_op::handle_shra, num_params: 0
    },
    OpCode {
        symbol: "JUMP", byte: 0x30,
        handler: handle_op::handle_jump, num_params: 1
    },
    OpCode {
        symbol: "IFZ", byte: 0x31,
        handler: handle_op::handle_ifz, num_params: 1
    },
    OpCode {
        symbol: "IFNZ", byte: 0x32,
        handler: handle_op::handle_ifnz, num_params: 1
    },
    OpCode {
        symbol: "CALL", byte: 0x33,
        handler: handle_op::handle_call, num_params: 1
    },
    OpCode {
        symbol: "RETURN", byte: 0x34,
        handler: handle_op::handle_return, num_params: 1
    },
];
