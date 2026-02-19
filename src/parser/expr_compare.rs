/*
parser/expr.rs: Helper function for simple comparison of expression Enums
Copyright (C) 2025-26 The CCIL Developers

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
            FunctionDeclaration(_, _, _) => Self::FunctionDeclaration,
            FunctionCall(_, _) => Self::FunctionCall,
            ForLoop(_, _) => Self::ForLoop,
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
            FunctionDeclaration => Self::FunctionDeclaration(
                Box::new(Expr::Empty), Box::new(Expr::Empty), Box::new(Expr::Empty)
            ),
            FunctionCall => Self::FunctionCall(Token::Dummy, Box::new(Expr::Empty)),
            ForLoop => Self::ForLoop(
                Box::new(Expr::Empty), Box::new(Expr::Empty)
            ),
            WhileLoop => Self::WhileLoop(Box::new(Expr::Empty), Box::new(Expr::Empty)),
            PrintStatement => Self::PrintStatement(Box::new(Expr::Empty)),
            ReturnStatement => Self::ReturnStatement(Box::new(Expr::Empty)),
            IfStatement => Self::IfStatement(Box::new(Expr::Empty), Box::new(Expr::Empty)),
        };
        return discriminant(self) == discriminant(&generic_expr);
    }
}