use crate::parser::{expr::Expr, rules::PARSE_RULE_TABLE, token::Token};
pub mod expr;
pub mod token;
pub mod rules;
pub mod handlers;

#[allow(unused)]
struct Parser {
    current_line: usize,
    tokens_to_process: Vec<Token>,
    expressions: Vec<Expr>
}

#[allow(unused)]
impl Parser {
    /// Prints parsing error message and exits with code 1. Function does not return
    fn raise_parsing_error(self, error_message: &str) {
        // Todo in future: create specific error message struct/enum to hold more complex info?
        eprintln!("Error at line {}: \n\n {}", self.current_line, error_message);
        std::process::exit(1);
    }

    fn parse() {
        let rule_table = PARSE_RULE_TABLE.clone();
    }
}
