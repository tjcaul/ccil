use crate::parser::{Parser, expr::Expr, token::Token};

#[allow(unused)]
type ParseHandler = fn(&mut Parser, &Token) -> Expr;

/// Defines an order in which tokens should be consumed.
/// Greater always implies greater precedence (i.e. should be consumed first).
/// This hierarchy is inspired by JavaScript's.
#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Precedence {
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
#[derive(Debug, Clone)]
pub struct ParseRule {
    pub handler: ParseHandler,
    precedence: Precedence
}

#[allow(unused)]
impl ParseRule {
    pub fn get_parse_rule(token: &Token) -> Option<Self> {
        // Import into namespace for brevity
        use Token::*;
        use Precedence::*;
        // For some reason we have to explicitly indicate type here instead of using alias ParseHandler
        let (handler, precedence): (fn(&mut Parser, &Token) -> Expr, Precedence) = match token {
            // Start of a group
            LeftParen | LeftCurly | LeftSquare => (Parser::grouping, Lowest),

            // Arithmetic
            Plus => (Parser::binary, Term),
            // For ambigious tokens like minus, we use a custom parsing rule and give it the highest precedence
            Minus => (Parser::minus, Unary),
            Star | Slash => (Parser::binary, Factor),

            // Bitwise
            Tilde => (Parser::unary, Unary),
            SingleAnd => (Parser::binary, BitwiseAnd),
            Carat => (Parser::binary, BitwiseXor),
            SingleOr => (Parser::binary, BitwiseOr),
            DoubleLessThan | DoubleGreaterThan => (Parser::binary, BitShift),

            // Variable Name (or function call)
            VarName(_) => (Parser::variable, Call),

            // Assignment
            Equals => (Parser::assignment, Assignment),

            // Boolean
            Bang => (Parser::unary, Unary),
            And => (Parser::binary, BooleanAnd),
            Or => (Parser::binary, BooleanOr),

            // Equality/Comparison
            DoubleEqual | BangEqual => (Parser::binary, Equality),
            GreaterThan | GreaterThanEqual | LessThan | LessThanEqual => (Parser::binary, Comparison),

            // Comma separated lists
            Comma => (Parser::comma_separated_list, Lowest),

            // Literals
            String(_) | Number(_) | Float(_) | Boolean(_) | Null => (Parser::literal, Lowest),

            // Keywords
            Var => (Parser::variable_declaration, Lowest),
            Func => (Parser::function_declaration, Lowest),
            For => (Parser::for_loop, Lowest),
            While => (Parser::while_loop, Lowest),
            Print => (Parser::print_statement, Lowest),
            Return => (Parser::return_statement, Lowest),
            If => (Parser::if_statement, Lowest),

            // The following tokens are "unexpected" here because they're only always consumed by other means:
            // RightParen RightCurly RightSquare Semicolon NewLine Dot Equals
            _ => { return None; }
        };

        return Some(Self {
            handler, precedence
        });
    }
}

