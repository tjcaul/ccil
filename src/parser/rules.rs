/*
parser/rules.rs: Defines token types and precedence
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

use crate::parser::{Parser, expr::Expr, token::Token};

#[allow(unused)]
pub type ParseRule = fn(&mut Parser, &Token) -> Expr;

/// Defines an order in which tokens should be consumed.
/// Greater always implies greater precedence (i.e. should be consumed first).
/// This hierarchy is inspired by JavaScript's.
#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Precedence {
    Lowest,
    Assignment,  // =
    BooleanOr,   // ||
    BooleanAnd,  // &&
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
    Literal,
    Grouping,
    Highest
}

impl Precedence {
    pub fn next_highest(&self) -> Self {
        use Precedence::*;
        match self {
            Lowest => Assignment,
            Assignment => BooleanOr,
            BooleanOr => BooleanAnd,
            BooleanAnd => BitwiseOr,
            BitwiseOr => BitwiseXor,
            BitwiseXor => BitwiseAnd,
            BitwiseAnd => Equality,
            Equality => Comparison,
            Comparison => BitShift,
            BitShift => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Literal,
            Literal => Grouping,
            Grouping => Highest,
            Highest => Highest
        }
    }
}

#[allow(unused)]
impl Token {
    pub fn get_parse_rule(&self) -> Option<ParseRule> {
        // Import into namespace for brevity
        use Token::*;
        let handler = match self {
            // Start of a group
            LeftParen | LeftCurly | LeftSquare => Parser::grouping,

            // Arithmetic
            Plus | Star | Slash | Percent => Parser::binary,
            // For ambigious tokens like minus, we use a custom parsing rule
            Minus => Parser::minus,

            // Bitwise
            Tilde => Parser::unary,
            SingleAnd | Carat | SingleOr | DoubleLessThan | DoubleGreaterThan => Parser::binary,

            // Variable Name (or function call)
            VarName(_) => Parser::variable,

            // Assignment
            Equals => Parser::assignment,

            // Boolean
            Bang => Parser::unary,
            And | Or => Parser::binary,

            // Equality/Comparison
            DoubleEqual | BangEqual | GreaterThan |
            GreaterThanEqual | LessThan | LessThanEqual => Parser::binary,

            // Comma separated lists
            Comma => Parser::comma_separated_list,

            // Literals
            String(_) | Number(_) | Float(_) | Boolean(_) | Null => Parser::literal,

            // Keywords all get their own handler
            Var => Parser::variable_declaration,
            Func => Parser::function_declaration,
            For => Parser::for_loop,
            While => Parser::while_loop,
            Print => Parser::print_statement,
            Return => Parser::return_statement,
            If => Parser::if_statement,

            // The following tokens are "unexpected" here because they're only always consumed by other means:
            // RightParen RightCurly RightSquare Semicolon NewLine Dot Equals
            _ => { return None; }
        };

        return Some(handler);
    }

    pub fn get_precedence(&self, has_prefix: bool) -> Precedence {
        use Token::*;
        use Precedence::*;
        match self {
            Comma | Var | Func | For | While | Print | Return | If => Lowest,
            LeftParen | LeftCurly | LeftSquare => Grouping,
            Plus => Term,
            // Minus is ambiguous
            Minus => {
                if has_prefix {
                    Term
                } else {
                    Unary
                }
            },
            Star | Slash | Percent => Factor,
            Bang | Tilde => Unary,
            SingleAnd => BitwiseAnd,
            Carat => BitwiseXor,
            SingleOr => BitwiseOr,
            DoubleLessThan | DoubleGreaterThan => BitShift,
            String(_) | Number(_) | Float(_) | Boolean(_) | Null => Literal,
            VarName(_) => Call,
            Equals => Assignment,
            And => BooleanAnd,
            Or => BooleanOr,
            DoubleEqual | BangEqual => Equality,
            GreaterThan | GreaterThanEqual | LessThan | LessThanEqual => Comparison,
            _ => Lowest
        }
    }
}
