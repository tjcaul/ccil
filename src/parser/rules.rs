use std::{collections::HashMap, sync::LazyLock};

use crate::parser::{handlers::grouping, token::Token};

#[allow(unused)]
type ParseHandler = fn();

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
    prefix: Option<ParseHandler>,
    infix: Option<ParseHandler>,
    precedence: Precedence
}

#[allow(unused)]
pub static PARSE_RULE_TABLE: LazyLock<HashMap<Token, ParseRule>> = std::sync::LazyLock::new(|| HashMap::from([
    (Token::LeftParen, ParseRule {prefix: Some(grouping), infix: None, precedence: Precedence::None})
]));
