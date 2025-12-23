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
    fn raise_parsing_error(self, error_message: String) -> ! {
        // Todo in future: create specific error message struct/enum to hold more complex info?
        eprintln!("Error at line {}: \n\n {}", self.current_line, error_message);
        std::process::exit(1);
    }

    fn parse_step(mut self) -> Option<Self> {
        let current_token = self.tokens_to_process.pop().unwrap();
        if current_token == Token::EOF {
            return None;
        }
        let previous_token = self.tokens_processed.last();

        let parse_rule = ParseRule::get_parse_rule(&current_token);
        let handler = match parse_rule.prefix {
            Some(val) => val,
            None => self.raise_parsing_error(format!("Unexpected token {:?}", current_token))
        };

        let expr;
        (expr, self) = handler(self);

        self.expressions.push(expr);
        self.tokens_processed.push(current_token);
        return Some(self);
    }

    fn parse_expect(mut self, expected: Token) -> Option<Self> {
        todo!()
    }
}
