use crate::parser::token::Token;

#[allow(unused)]
pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
}

#[allow(unused)]
pub struct Unary {
    operand: Box<Expr>,
    operator: Token,
}

#[allow(unused)]
pub struct Binary {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token,
}

#[allow(unused)]
pub struct Grouping {
    expression: Box<Expr>,
}

#[allow(unused)]
pub struct Literal {
    value: Token,
}
