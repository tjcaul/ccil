use crate::parser::token::Token;

pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
}

pub struct Unary {
    operand: Box<Expr>,
    operator: Token,
}

pub struct Binary {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token,
}

pub struct Grouping {
    expression: Box<Expr>,
}

pub struct Literal {
    value: Token,
}
