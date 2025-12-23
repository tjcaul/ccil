use std::cmp::min;

use ordered_float::OrderedFloat;

/// Helper function to remove comments along with whitespace
fn trim_start_comments(remaining_block: &str) -> &str {
    let non_newline_whitespace = |c: char| c.is_whitespace() && c != '\n';
    let mut trimming = remaining_block.trim_matches(non_newline_whitespace);
    if trimming.len() > 1 && &trimming[0..2] == "//" {
        // get rid of comments, up till next newline
        trimming = &trimming[trimming.find('\n').unwrap_or(trimming.len())..];
    }
    return trimming;
}

/// Helper function to tokenize string literal
fn tokenize_string(remaining_block: &str) -> (Token, usize) {
    let closing_index = match remaining_block.find("\"") {
        Some(val) => val,
        None => panic!("No closing quote found when parsing string literal")
    };

    // Need to account for two quotation marks in size
    return (Token::String(remaining_block[0..closing_index].to_string()), closing_index + 2);
}

/// Helper function to tokenize numbers or floats, returning appropriate token type
fn tokenize_number_or_float(remaining_block: &str) -> (Token, usize) {
    let full_literal = preprocess_until_interrupted(remaining_block);

    match full_literal.parse::<i32>() {
        Ok(val) => return (Token::Number(val), full_literal.len()),
        Err(_) => {}
    };

    match full_literal.parse::<f64>() {
        Ok(val) => return (Token::Float(OrderedFloat(val)), full_literal.len()),
        Err(_) => panic!("Illegal number detected")
    }
}

/// Helper function to preprocess (not tokenize) whitespace, semicolon or comment-separated info
fn preprocess_until_interrupted(remaining_block: &str) -> &str {
    let next_whitespace = remaining_block.find(char::is_whitespace).unwrap_or(remaining_block.len());
    let next_comment = remaining_block.find("//").unwrap_or(remaining_block.len());
    let next_semicolon = remaining_block.find(";").unwrap_or(remaining_block.len());

    let closing_index = min(next_whitespace, min(next_comment, next_semicolon));

    return &remaining_block[0..closing_index];
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    // Single-char
    LeftParen, RightParen,
    LeftCurly, RightCurly,
    LeftSquare, RightSquare,
    Comma,
    Dot,
    Minus, Plus, Slash, Star,
    Semicolon,
    Equal,
    SingleAnd, SingleOr, Tilde, Carat,
    DoubleLessThan, DoubleGreaterThan,

    // Comparison (single or double char)
    DoubleEqual, BangEqual,
    GreaterThan, GreaterThanEqual,
    LessThan, LessThanEqual,

    // Boolean
    Bang, And, Or,

    // Literal
    // We need 64 bits of float for guaranteed lossless conversion between num and float
    // We use a wrapper provided by the OrderedFloat crate for better equality checks and hashability
    String(String), Number(i32), Float(OrderedFloat<f64>), Boolean(bool),

    // Keywords
    Var, Func, For, While, Print, Return, If, Null,

    // Misc
    VarName(String), NewLine, EOF
}

#[allow(unused)]
impl Token {
    pub fn needs_value(self) -> bool {
        matches!(self, Token::String(_)) ||
        matches!(self, Token::Number(_)) ||
        matches!(self, Token::Float(_)) ||
        matches!(self, Token::Boolean(_))
    }

    pub fn get_string(self) -> Option<String> {
        match self {
            Token::String(s) => Some(s),
            _ => None
        }
    }

    pub fn get_number(self) -> Option<i32> {
        match self {
            Token::Number(n) => Some(n),
            _ => None
        }
    }

    pub fn get_float(self) -> Option<f64> {
        match self {
            Token::Float(f) => Some(*f),
            _ => None
        }
    }

    pub fn get_boolean(self) -> Option<bool> {
        match self {
            Token::Boolean(b) => Some(b),
            _ => None
        }
    }

    pub fn get_var_name(self) -> Option<String> {
        match self {
            Token::VarName(v) => Some(v),
            _ => None
        }
    }

    /// Returns a token from the remaining text, following grammar rules.
    /// Also returns a slice of the remaining string to process.
    /// Input should have leading whitespace removed,
    /// and output is guaranteed to have leading whitespace removed.
    fn scan_token(slice_to_end: &str) -> (Self, &str) {
        if slice_to_end.is_empty() {
            return (Token::EOF, "");
        }

        let (found_token, length) = match &slice_to_end.chars().next().unwrap() {
            // Unambiguous single-chars are the easiest; we simply match directly.
            '(' => (Token::LeftParen, 1),
            ')' => (Token::RightParen, 1),
            '{' => (Token::LeftCurly, 1),
            '}' => (Token::RightCurly, 1),
            '[' => (Token::LeftSquare, 1),
            ']' => (Token::RightSquare, 1),
            ',' => (Token::Comma, 1),
            '.' => (Token::Dot, 1),
            '-' => (Token::Minus, 1),
            '+' => (Token::Plus, 1),
            // Guaranteed to be fine since we treat comments as whitespace
            '/' => (Token::Slash, 1),
            '*' => (Token::Star, 1),
            ';' => (Token::Semicolon, 1),
            '~' => (Token::Tilde, 1),
            '^' => (Token::Carat, 1),
            '\n' => (Token::NewLine, 1),

            // These leading tokens could vary in meaning based on second token.
            '=' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::DoubleEqual, 2)
                } else {
                    (Token::Equal, 1)
                }
            }
            '!' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::BangEqual, 2)
                } else {
                    (Token::Bang, 1)
                }
            }
            '<' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::GreaterThanEqual, 2)
                } else if slice_to_end.len() > 1 && &slice_to_end[1..2] == "<" {
                    (Token::DoubleGreaterThan, 2)
                } else {
                    (Token::GreaterThan, 1)
                }
            }
            '>' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::LessThanEqual, 2)
                } else if slice_to_end.len() > 1 && &slice_to_end[1..2] == ">" {
                    (Token::DoubleLessThan, 2)
                } else {
                    (Token::LessThan, 1)
                }
            }
            '&' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "&" {
                    (Token::And, 2)
                } else {
                    (Token::SingleAnd, 1)
                }
            }
            '|' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "|" {
                    (Token::Or, 2)
                } else {
                    (Token::SingleOr, 1)
                }
            }

            // Literals can be done via some matching trickery
            // For strings, just detect opening quote and delegate to function from there
            '"' => tokenize_string(&slice_to_end[1..]),

            // For numbers and floats, detect starting digit
            '0' ..= '9' => tokenize_number_or_float(slice_to_end),

            // For other keywords (and true and false), instead go until next whitespace and match
            _ => match preprocess_until_interrupted(slice_to_end) {
                "var" => (Token::Var, 3),
                "func" => (Token::Func, 4),
                "for" => (Token::For, 3),
                "while" => (Token::While, 5),
                "print" => (Token::Print, 5),
                "return" => (Token::Return, 6),
                "if" => (Token::If, 2),
                "true" => (Token::Boolean(true), 4),
                "false" => (Token::Boolean(false), 5),
                "null" => (Token::Null, 4),
                _ => panic!("Unknown keyword found while tokenizing")
            }
        };

        return (found_token, trim_start_comments(&slice_to_end[length..]));
    }

    pub fn full_scan(input: &str) -> Vec<Self> {
        let mut remaining = input;
        let mut retval = Vec::<Token>::new();
        while retval.last().unwrap_or(&Token::LeftParen) != &Token::EOF {
            let (next_token, unused_slice) = Token::scan_token(remaining);
            retval.push(next_token);
            remaining = unused_slice;
        }
        retval.reverse(); // lets us use it as a stack
        return retval;
    }
}
