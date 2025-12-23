use crate::parser::{token::Token};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
}
