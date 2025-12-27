use crate::parser::{expr::Expr, rules::ParseRule, token::Token};
pub mod expr;
pub mod token;
pub mod rules;

#[allow(unused)]
#[derive(Debug)]
pub struct Parser {
    current_line: usize,
    tokens_to_process: Vec<Token>,
    floating_expressions: Vec<Expr>,
    pub expressions: Vec<Expr>
}

#[allow(unused)]
impl Parser {
    pub fn new(tokens_to_process: Vec<Token>) -> Self {
        Self { 
            current_line: 1,
            tokens_to_process,
            floating_expressions: Vec::<Expr>::new(),
            expressions: Vec::<Expr>::new()
        }
    }

    /// Prints parsing error message and exits with code 1. Function does not return
    fn raise_parsing_error(&self, error_message: String) -> ! {
        // Todo in future: create specific error message struct/enum to hold more complex info?
        eprintln!("Error on line {}:\n\n{}", self.current_line, error_message);
        std::process::exit(1);
    }

    /// Steps forward a token in the parser and adds an expression if necessary.
    /// If EOF is reached, return do nothing.
    pub fn parse_step(&mut self) {
        let current_token = self.consume_and_return();
        // TODO: maybe just consider moving this return to later when we pattern match for none?
        if current_token == Token::EOF {
            return;
        }
        if current_token == Token::Semicolon {
            self.expressions.push(match self.floating_expressions.len() {
                0 => Expr::Empty,
                1 => self.floating_expressions.pop().unwrap(),
                _ => self.raise_parsing_error("Illegal expression".to_owned())
            });
            return;
        }
        if current_token == Token::NewLine {
            self.current_line += 1;
            return;
        }

        let parse_rule = match ParseRule::get_parse_rule(&current_token) {
            Some(val) => val,
            None => self.raise_parsing_error(format!("Unexpected token {:?}", current_token))
        };

        let expr = (parse_rule.handler)(self, &current_token);

        self.floating_expressions.push(expr);
    }

    /// Pops the top token from the stack and returns it.
    fn consume_and_return(&mut self) -> Token {
        self.tokens_to_process.pop().unwrap_or(Token::EOF)
    }

    /// Consumes the token and errors out if encountering another.
    /// Ignores fields. Shouldn't really be used with any fields
    /// but just set to some dummy value for expected.
    fn consume_expected(&mut self, expected: Token) {
        let token = self.consume_and_return();
        if std::mem::discriminant(&token) != std::mem::discriminant(&expected) {
            self.raise_parsing_error(format!("Expected token {:?}, got token {:?}", expected, token));
        }
    }

    pub fn full_parse(&mut self) {
        while !self.tokens_to_process.is_empty() {
            self.parse_step();
        }
    }

    /// Emits bytecode of completed expressions into a chunk.
    fn emit_bytecode(self) -> Vec<u8> {
        todo!()
    }
}
