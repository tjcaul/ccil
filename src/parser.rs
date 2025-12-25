use crate::parser::{expr::Expr, rules::ParseRule, token::Token};
pub mod expr;
pub mod token;
pub mod rules;
pub mod handlers;

#[allow(unused)]
#[derive(Debug)]
pub struct Parser {
    current_line: usize,
    tokens_to_process: Vec<Token>,
    tokens_processed: Vec<Token>,
    expressions: Vec<Expr>
}

#[allow(unused)]
impl Parser {
    pub fn new(tokens_to_process: Vec<Token>) -> Self {
        Self { 
            current_line: 0,
            tokens_to_process,
            tokens_processed: Vec::<Token>::new(),
            expressions: Vec::<Expr>::new()
        }
    }

    /// Prints parsing error message and exits with code 1. Function does not return
    fn raise_parsing_error(&self, error_message: String) -> ! {
        // Todo in future: create specific error message struct/enum to hold more complex info?
        eprintln!("Error at line {}: \n\n {}", self.current_line, error_message);
        std::process::exit(1);
    }

    /// Steps forward a token in the parser and adds any resultant expressions.
    /// If EOF is reached, return EOF and do nothing else.
    fn parse_step(&mut self) {
        let current_token = self.consume_and_return();
        if current_token == Token::EOF {
            return;
        }
        // no need to unwrap now
        // let previous_expression = self.expressions.last();

        let parse_rule = ParseRule::get_parse_rule(&current_token);
        let handler = match parse_rule.prefix {
            Some(val) => val,
            None => self.raise_parsing_error(format!("Unexpected token {:?}", current_token))
        };

        let expr = handler(self);

        self.expressions.push(expr);
        self.tokens_processed.push(current_token);
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

    /// Emits bytecode of completed expressions into a chunk.
    fn emit_bytecode(self) -> Vec<u8> {
        todo!()
    }
}
