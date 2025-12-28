use crate::parser::{expr::Expr, token::Token};
pub mod expr;
pub mod expr_compare;
pub mod token;
pub mod rules;

#[allow(unused)]
#[derive(Debug, Clone)]
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

    /// Steps forward a token in the parser and adds a parentless expression.
    /// If EOF is reached, return do nothing.
    pub fn parse_step(&mut self) {
        let expr = self.generate_until_semicolon();
        self.consume_expected(Token::Semicolon);
        self.expressions.push(expr);
    }
    
    /// Performs parses until out of tokens to parse.
    pub fn full_parse(&mut self) {
        while self.peek() != Token::EOF {
            self.parse_step();
        }
    }

    /// Tells us the next token without popping it from the stack.
    fn peek(&mut self) -> Token {
        self.tokens_to_process
            .iter().rev().find(|&x| x != &Token::NewLine)
            .unwrap_or(&Token::EOF)
            .clone()
    }

    /// Pops the top token from the stack and returns it. On NewLine, increments the counter and tries again.
    fn consume_and_return(&mut self) -> Token {
        let token = self.tokens_to_process.pop().unwrap_or(Token::EOF);
        if token == Token::NewLine {
            self.current_line += 1;
            return self.consume_and_return();
        }
        return token;
    }

    /// Consumes the token and errors out if encountering another.
    /// Ignores fields. Shouldn't really be used with any fields
    /// but just set to some dummy value for expected.
    fn consume_expected(&mut self, expected: Token) -> Token {
        let token = self.consume_and_return();
        if std::mem::discriminant(&token) != std::mem::discriminant(&expected) {
            self.raise_parsing_error(format!("Expected token {:?}, got token {:?}", expected, token));
        }
        return token;
    }

    /// Emits bytecode of completed expressions into a chunk.
    fn emit_bytecode(self) -> Vec<u8> {
        todo!()
    }
}
