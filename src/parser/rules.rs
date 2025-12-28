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
    Primary
}

#[allow(unused)]
impl Token {
    pub fn get_parse_rule(&self) -> Option<ParseRule> {
        // Import into namespace for brevity
        use Token::*;
        // For some reason we have to explicitly indicate type here instead of using alias ParseRule
        let handler : fn(&mut Parser, &Token) -> Expr = match self {
            // Start of a group
            LeftParen | LeftCurly | LeftSquare => Parser::grouping,

            // Arithmetic
            Plus | Star | Slash => Parser::binary,
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
            LeftParen | LeftCurly | LeftSquare | Comma |
            String(_) | Number(_) | Float(_) | Boolean(_) | Null |
            Var | Func | For | While | Print | Return | If => Lowest,
            Plus => Term,
            // Minus is ambiguous
            Minus => {
                if has_prefix {
                    Term;
                }
                Unary
            },
            Star | Slash => Factor,
            Bang | Tilde => Unary,
            SingleAnd => BitwiseAnd,
            Carat => BitwiseXor,
            SingleOr => BitwiseOr,
            DoubleLessThan | DoubleGreaterThan => BitShift,
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
