use crate::{compiler::Compiler, constants::{GENERIC_COMPILE_ERROR, fileno_const, type_id_const}, parser::{expr::Expr, token::Token}, vm::opcode::Argument};

impl Compiler<'_> {
    pub fn compile_literal(&self, token: &Token) -> (Vec<u8>, Argument) {
        let (val, type_id) = match token {
            Token::Number(val) => (*val, type_id_const::NUMBER),
            Token::String(val) => {
                let string_id = self.find_or_insert_string(val);
                (string_id as i32, type_id_const::STRING)
            }
            Token::Boolean(val) => {
                if *val {
                    (1, type_id_const::BOOLEAN)
                } else {
                    (0, type_id_const::BOOLEAN)
                }
            }
            Token::Float(_) => todo!(),
            Token::Null => (0, type_id_const::NULL),
            _ => panic!("{}", GENERIC_COMPILE_ERROR)
        };

        (self.emit_constant(val), type_id)
    }

    pub fn compile_binary(&self, token: &Token, left: &Expr, right: &Expr) -> (Vec<u8>, Argument) {
        let mut retval = Vec::<u8>::new();
        use Token::*;
        match token {
            Equals => {
                let (mut compile_right, type_id) = self.compile_one(right);
                retval.append(&mut compile_right);

                let var_name = left.get_token().get_var_name().unwrap();

                let (var_id, _) = self.get_or_insert(var_name);
                self.set_inferred_type(var_name, type_id);

                let mut assignment: Vec<u8> = self.emit_assignment(var_id, type_id);
                retval.append(&mut assignment);

                (retval, type_id_const::UNKNOWN) // Assignments don't push anything to the stack
            },
            _ => {
                let (mut compile_left, left_type_id) = self.compile_one(left);
                let (mut compile_right, right_type_id) = self.compile_one(right);
                retval.append(&mut compile_left);
                retval.append(&mut compile_right);
                let (instr, type_id) = match token {
                    Plus => {
                        match (left_type_id, right_type_id) {
                            (type_id_const::NUMBER, type_id_const::NUMBER) => {
                                ("ADD", type_id_const::NUMBER)
                            },
                            _ => todo!() // e.g. string plus number or string plus string
                        }
                    }
                    Minus => {
                        if (left_type_id, right_type_id) == (type_id_const::NUMBER, type_id_const::NUMBER) {
                            ("SUB", type_id_const::NUMBER)
                        } else {
                            panic!("{}", GENERIC_COMPILE_ERROR)
                        }
                    }
                    Star => {
                        match (left_type_id, right_type_id) {
                            (type_id_const::NUMBER, type_id_const::NUMBER) => {
                                ("MUL", type_id_const::NUMBER)
                            },
                            _ => todo!() // e.g. string times number
                        }
                    }
                    Slash => {
                        match (left_type_id, right_type_id) {
                            (type_id_const::NUMBER, type_id_const::NUMBER) => {
                                ("DIV", type_id_const::NUMBER)
                            },
                            _ => todo!() // float division
                        }
                    }
                    Percent => {
                        if (left_type_id, right_type_id) == (type_id_const::NUMBER, type_id_const::NUMBER) {
                            ("MOD", type_id_const::NUMBER)
                        } else {
                            panic!("{}", GENERIC_COMPILE_ERROR)
                        }
                    }
                    // TODO: Bitwise, boolean, comparison
                    _ => panic!("{}", GENERIC_COMPILE_ERROR)
                };
                let mut instr_op = self.emit_instr(instr);
                retval.append(&mut instr_op);
                (retval, type_id)
            }
        }
    }

    pub fn compile_print(&self, expr: &Expr) -> (Vec<u8>, Argument) {
        let mut retval = Vec::<u8>::new();

        let (mut compile_expr, type_id) = self.compile_one(expr);
        retval.append(&mut compile_expr);


        let mut write = match type_id {
            type_id_const::STRING => self.emit_writes(fileno_const::STDOUT),
            _ => self.emit_write(fileno_const::STDOUT)
        };
        retval.append(&mut write);

        return (retval, type_id_const::UNKNOWN);
    }

    pub fn compile_variable(&self, token: &Token) -> (Vec<u8>, Argument) {
        let var_name = match token.get_var_name() {
            Some(val) => val,
            None => panic!("{}", "Attempted illegal assignment")
        };

        let (var_id, type_id) = self.get_or_insert(var_name);

        return (self.emit_load(var_id), type_id);
    }
}
