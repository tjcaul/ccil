use crate::parser::{Parser, expr::Expr, token::Token};

#[allow(unused)]
type ParseHandler = fn(&mut Parser, token: &Token) -> Expr;

/// Defines an order in which tokens should be consumed.
/// Greater always implies greater precedence (i.e. should be consumed first).
/// This hierarchy is inspired by JavaScript's.
#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Precedence {
    None,
    Assignment,  // =
    Or,          // ||
    And,         // &&
    BitwiseOr,   // |
    BitwiseXor,  // ^
    BitwiseAnd,  // &
    Equality,    // == !=
    Comparison,  // > < >= <=
    BitShift,    // << >>
    Term,        // + -
    Factor,      // * /
    Unary,       // ! - ~
    Call,
    Primary
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ParseRule {
    pub handler: Option<ParseHandler>,
    precedence: Precedence
}

#[allow(unused)]
impl ParseRule {
    pub fn get_parse_rule(token: &Token) -> Self {
        match token {
            Token::LeftParen => Self {handler: Some(Parser::grouping), precedence: Precedence::None},
            _ => todo!()
        }
    }
}

