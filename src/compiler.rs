/*
constants.rs: The CCIL Bytecode Compiler
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

use std::cell::RefCell;

use rustc_hash::FxHashMap;

use crate::{parser::{expr::Expr, token::Token}, vm::{opcode::{Argument, OpCodeLookup}}};

pub mod emitters;
pub mod rules;

pub struct Compiler<'a> {
    lookup: OpCodeLookup<'a>,
    variables: RefCell<FxHashMap<Token, Argument>>,
    string_map: RefCell<FxHashMap<String, usize>>,
    pub string_pool: RefCell<Vec<u8>>
}

impl Compiler<'_> {
    pub fn new() -> Self {
        Self {
            lookup: OpCodeLookup::new(),
            variables: RefCell::new(FxHashMap::default()),
            string_map: RefCell::new(FxHashMap::default()),
            string_pool: RefCell::new(Vec::new())
        }
    }

    pub fn compile(&self, expressions: &Vec<Expr>) -> Vec<u8> {
        let mut retval = Vec::<u8>::new();
        for expression in expressions {
            let (mut compiled, _) = self.compile_one(expression);
            retval.append(&mut compiled);
        }
        return retval;
    }

    fn compile_one(&self, expression: &Expr) -> (Vec<u8>, Argument) {
        let mut retval = Vec::<u8>::new();
        use Expr::*;
        let (mut compiled, type_id) = match expression {
            Literal(token) => self.compile_literal(token),
            Binary(token, left, right) => self.compile_binary(token, left, right),

            PrintStatement(expr) => self.compile_print(expr),
            _ => todo!()
        };
        retval.append(&mut compiled);
        (retval, type_id)
    }

    fn get_or_insert(&self, val: &Token) -> Argument {
        let mut borrowed_variables = self.variables.borrow_mut();
        let default_sp = borrowed_variables.len() as Argument;
        match borrowed_variables.get(val) {
            Some(val) => val.clone(),
            None => {
                borrowed_variables.insert(val.clone(), default_sp);
                default_sp
            },
        }
    }

    // fn to_variable_value(&self, expression: &Expr) -> VariableValue {
    //     use Expr::*;
    //     match expression {
    //         Literal(token) => match token {
    //             Token::String(val) => VariableValue::StringLiteral(self.find_or_insert_string(val)),
    //             Token::Number(val) => VariableValue::Number(*val),
    //             Token::Float(val) => VariableValue::Float(*val),
    //             Token::Boolean(val) => VariableValue::Boolean(*val),
    //             Token::Null => VariableValue::Null,
    //             _ => panic!("{}", GENERIC_COMPILE_ERROR)
    //         },
    //         _ => panic!("{}", GENERIC_COMPILE_ERROR)
    //     }
    // }

    fn find_or_insert_string(&self, string_value: &String) -> usize {
        let mut borrowed_string_map = self.string_map.borrow_mut();
        let mut borrowed_string_pool = self.string_pool.borrow_mut();
        if let Some(val) = borrowed_string_map.get(string_value) {
            *val
        } else {
            let val = borrowed_string_pool.len();
            borrowed_string_map.insert(string_value.to_string(), val);
            for byte in string_value.as_bytes() {
                borrowed_string_pool.push(*byte);
            }
            borrowed_string_pool.push(0); // null terminator
            val
        }
    }
}


