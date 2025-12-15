use std::collections::BTreeMap;
use crate::vm::handle_op;
use crate::vm::handle_op::OpcodeHandler;

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
        symbol: "CONSTANT", byte: 0x03,
        handler: handle_op::handle_constant, num_params: 1
    },
    OpCode {
        symbol: "RETURN", byte: 0x01,
        handler: handle_op::handle_return, num_params: 1
    },
    OpCode {
        symbol: "ADD", byte: 0x02,
        handler: handle_op::handle_add, num_params: 2
    },
];
