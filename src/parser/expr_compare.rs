use std::mem::discriminant;

use crate::parser::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum ExprType {
    Empty,
    Unary,
    Binary,
    Grouping,
    CurlyGrouping,
    SquareGrouping,
    Literal,
    CommaSeparatedList,
    Subexprs,
    Variable,
    VaribleDeclaration,
    FunctionDeclaration,
    FunctionCall,
    ForLoop,
    WhileLoop,
    PrintStatement,
    ReturnStatement,
    IfStatement,
}

impl ExprType {
    pub fn from_expr(expr: &Expr) -> Self {
        use Expr::*;
        match expr {
            Empty => Self::Empty,
            Unary(_, _) => Self::Unary,
            Binary(_, _, _) => Self::Binary,
            Grouping(_) => Self::Grouping,
            CurlyGrouping(_) => Self::CurlyGrouping,
            SquareGrouping(_) => Self::SquareGrouping,
            Literal(_) => Self::Literal,
            CommaSeparatedList(_) => Self::CommaSeparatedList,
            Subexprs(_) => Self::Subexprs,
            Variable(_) => Self::Variable,
            VaribleDeclaration(_) => Self::VaribleDeclaration,
            FunctionDeclaration(_, _, _) => Self::FunctionDeclaration,
            FunctionCall(_, _) => Self::FunctionCall,
            ForLoop(_, _, _, _) => Self::ForLoop,
            WhileLoop(_, _) => Self::WhileLoop,
            PrintStatement(_) => Self::PrintStatement,
            ReturnStatement(_) => Self::ReturnStatement,
            IfStatement(_, _) => Self::ReturnStatement
        }
    }
}

impl Expr {
    pub fn is_type(&self, compare_type: &ExprType) -> bool {
        use ExprType::*;
        let generic_expr = match compare_type {
            Empty => Self::Empty,
            Unary => Self::Unary(Token::Dummy, Box::new(Expr::Empty)),
            Binary => Self::Binary(Token::Dummy, Box::new(Expr::Empty), Box::new(Expr::Empty)),
            Grouping => Self::Grouping(Box::new(Expr::Empty)),
            CurlyGrouping => Self::CurlyGrouping(Box::new(Expr::Empty)),
            SquareGrouping => Self::SquareGrouping(Box::new(Expr::Empty)),
            Literal => Self::Literal(Token::Dummy),
            CommaSeparatedList => Self::CommaSeparatedList(Vec::new()),
            Subexprs => Self::Subexprs(Vec::new()),
            Variable => Self::Variable(Token::Dummy),
            VaribleDeclaration => Self::VaribleDeclaration(Box::new(Expr::Empty)),
            FunctionDeclaration => Self::FunctionDeclaration(
                Box::new(Expr::Empty), Box::new(Expr::Empty), Box::new(Expr::Empty)
            ),
            FunctionCall => Self::FunctionCall(Token::Dummy, Box::new(Expr::Empty)),
            ForLoop => Self::ForLoop(
                Box::new(Expr::Empty), Box::new(Expr::Empty), Box::new(Expr::Empty), Box::new(Expr::Empty)
            ),
            WhileLoop => Self::WhileLoop(Box::new(Expr::Empty), Box::new(Expr::Empty)),
            PrintStatement => Self::PrintStatement(Box::new(Expr::Empty)),
            ReturnStatement => Self::ReturnStatement(Box::new(Expr::Empty)),
            IfStatement => Self::IfStatement(Box::new(Expr::Empty), Box::new(Expr::Empty)),
        };
        return discriminant(self) == discriminant(&generic_expr);
    }
}