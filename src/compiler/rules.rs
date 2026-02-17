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
        match token {
            Token::Equals => {
                let var_id = self.get_or_insert(left.get_token());
                
                let (mut compile_right, type_id) = self.compile_one(right);
                retval.append(&mut compile_right);
                let mut assignment = self.emit_assignment(var_id, type_id);
                retval.append(&mut assignment);

                (retval, type_id_const::UNKNOWN) // Assignments don't push anything to the stack
            },
            _ => panic!("{}", GENERIC_COMPILE_ERROR)
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
}
