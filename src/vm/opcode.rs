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
        let mut symbol_lookup = BTreeMap::new();
        let mut byte_lookup = Vec::new();
        byte_lookup.resize(u8::MAX as usize - u8::MIN as usize + 1, None);

        // Add opcodes to tables
        for i in 0..OPCODES.len() {
            let opcode = &OPCODES[i];
            byte_lookup[opcode.byte as usize] = Some(opcode);
            symbol_lookup.insert(opcode.symbol, opcode);
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
        symbol: "CONSTANT", byte: 0x01,
        handler: handle_op::handle_constant, num_params: 1
    },
    OpCode {
        symbol: "POP", byte: 0x02,
        handler: handle_op::handle_pop, num_params: 0
    },
    OpCode {
        symbol: "COPY", byte: 0x03,
        handler: handle_op::handle_copy, num_params: 1
    },
    OpCode {
        symbol: "SWAP", byte: 0x04,
        handler: handle_op::handle_swap, num_params: 0
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
        symbol: "BOR", byte: 0x20,
        handler: handle_op::handle_bor, num_params: 0
    },
    OpCode {
        symbol: "BAND", byte: 0x20,
        handler: handle_op::handle_band, num_params: 0
    },
    OpCode {
        symbol: "BXOR", byte: 0x20,
        handler: handle_op::handle_bxor, num_params: 0
    },
    OpCode {
        symbol: "NOT", byte: 0x20,
        handler: handle_op::handle_not, num_params: 0
    },
    OpCode {
        symbol: "OR", byte: 0x20,
        handler: handle_op::handle_or, num_params: 0
    },
    OpCode {
        symbol: "AND", byte: 0x20,
        handler: handle_op::handle_and, num_params: 0
    },
    OpCode {
        symbol: "XOR", byte: 0x20,
        handler: handle_op::handle_xor, num_params: 0
    },
];
