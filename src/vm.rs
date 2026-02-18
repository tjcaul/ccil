/*
vm.rs: The CCIL Virtual Machine
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

use std::{cell::RefCell, fs::File};

use rustc_hash::FxHashMap;

use crate::compiler::VariableId;
use crate::{dprint, dprintln};
use crate::vm::{chunk::Chunk, opcode::OpCodeLookup, stack::{Stack, StackPointer, VecStack}, variable_value::VariableValue};

pub mod chunk;
pub mod handle_op;
pub mod opcode;
pub mod stack;
pub mod variable_value;


pub struct VirtualMachine<'a, 'b> {
    lookup: OpCodeLookup<'a>,
    stack: VecStack,
    variables: FxHashMap<VariableId, VariableValue>,
    string_pool: &'b RefCell<Vec<u8>>,
    opened_files: Vec<File>
}

impl<'b> VirtualMachine<'_, 'b> {
    pub fn new(string_pool: &'b RefCell<Vec<u8>>) -> Self {
        Self {
            lookup: OpCodeLookup::new(),
            stack: VecStack::new(),
            variables: FxHashMap::default(),
            string_pool,
            opened_files: Vec::new()
        }
    }

    pub fn execute(&mut self, chunk_to_execute: Vec<u8>) {
        let mut offset = 0;

        while offset < chunk_to_execute.len() {
            // Get opcode at current pos (guaranteed to be opcode by invariant)
            dprint!("{} ", offset);
            let chunk_code = match self.lookup.from_byte(chunk_to_execute[offset]) {
                Some(opcode) => opcode,
                None => panic!("Unknown opcode with value 0x{:02x}", chunk_to_execute[offset])
            };
            
            let mut args = Vec::<StackPointer>::new();
            for i in 0..chunk_code.num_params {
                args.push(chunk_to_execute.read_arg(offset + 1 + 4*i) as StackPointer);
            }

            // Run handler for op, we get next offset
            match (chunk_code.handler)(self, &args, offset) {
                Ok(Some(new_offset)) => { offset = new_offset; },
                Ok(None) => { break; }, // program exited
                Err(err) => { panic!("Error at chunk offset {}: {}", offset, err); }
            }
            dprintln!("\t{:?}", self.stack);
        }
    }

    pub fn get_string(&self, start_index: usize) -> String {
        let borrowed_string_pool = self.string_pool.borrow();
        let mut string_bytes = Vec::<u8>::new();
        let mut i = start_index;
        while borrowed_string_pool[i] != 0 {
            string_bytes.push(borrowed_string_pool[i]);
            i += 1
        }
        String::from_utf8(string_bytes).unwrap()
    }
}
