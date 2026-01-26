/*
parser/token.rs: CCIL source file tokenizer
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

use std::{cmp::min};

use ordered_float::OrderedFloat;

/// Helper function to remove comments along with whitespace
fn trim_start_comments(remaining_block: &str) -> &str {
    let non_newline_whitespace = |c: char| c.is_whitespace() && c != '\n';
    let mut trimming = remaining_block.trim_matches(non_newline_whitespace);
    if trimming.starts_with("//") {
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
    let full_literal = preprocess_number_or_float(remaining_block);

    match full_literal.parse::<i32>() {
        Ok(val) => return (Token::Number(val), full_literal.len()),
        Err(_) => {}
    };

    match full_literal.parse::<f64>() {
        Ok(val) => return (Token::Float(OrderedFloat(val)), full_literal.len()),
        Err(_) => panic!("Illegal number detected")
    }
}

/// Helper function to identify (but not tokenize) variable names or keywords
/// as each character must be either ASCII alphanumeric or the underscore
fn preprocess_keyword_or_varname(remaining_block: &str) -> &str {
    let closing_index = remaining_block.find(|c: char| !c.is_ascii_alphanumeric() && c != '_').unwrap_or(remaining_block.len());

    return &remaining_block[0..closing_index];
}

/// Helper function to preprocess (not tokenize) whitespace, semicolon or comment-separated info
fn preprocess_number_or_float(remaining_block: &str) -> &str {
    // use this hacky solution until they finally stabilize the pattern api
    let min_char_index = remaining_block.find(|c: char|
        !c.is_ascii_digit() && c != '.'
    ).unwrap_or(remaining_block.len());
    let next_comment = remaining_block.find("//").unwrap_or(remaining_block.len());

    let closing_index = min(min_char_index, next_comment);

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
    Equals,
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
    VarName(String), NewLine, EOF,

    Dummy // dummy value used for satisfying some fields
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
        use Token::*;
        if slice_to_end.is_empty() {
            return (EOF, "");
        }

        let (found_token, length) = match &slice_to_end.chars().next().unwrap() {
            // Unambiguous single-chars are the easiest; we simply match directly.
            '(' => (LeftParen, 1),
            ')' => (RightParen, 1),
            '{' => (LeftCurly, 1),
            '}' => (RightCurly, 1),
            '[' => (LeftSquare, 1),
            ']' => (RightSquare, 1),
            ',' => (Comma, 1),
            '.' => (Dot, 1),
            '-' => (Minus, 1),
            '+' => (Plus, 1),
            // Guaranteed to be fine since we treat comments as whitespace
            '/' => (Slash, 1),
            '*' => (Star, 1),
            ';' => (Semicolon, 1),
            '~' => (Tilde, 1),
            '^' => (Carat, 1),
            '\n' => (NewLine, 1),

            // These leading tokens could vary in meaning based on second token.
            '=' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (DoubleEqual, 2)
                } else {
                    (Equals, 1)
                }
            }
            '!' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (BangEqual, 2)
                } else {
                    (Bang, 1)
                }
            }
            '<' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (GreaterThanEqual, 2)
                } else if slice_to_end.len() > 1 && &slice_to_end[1..2] == "<" {
                    (DoubleGreaterThan, 2)
                } else {
                    (GreaterThan, 1)
                }
            }
            '>' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (LessThanEqual, 2)
                } else if slice_to_end.len() > 1 && &slice_to_end[1..2] == ">" {
                    (DoubleLessThan, 2)
                } else {
                    (LessThan, 1)
                }
            }
            '&' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "&" {
                    (And, 2)
                } else {
                    (SingleAnd, 1)
                }
            }
            '|' => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "|" {
                    (Or, 2)
                } else {
                    (SingleOr, 1)
                }
            }

            // Literals can be done via some matching trickery
            // For strings, just detect opening quote and delegate to function from there
            '"' => tokenize_string(&slice_to_end[1..]),

            // For numbers and floats, detect starting digit
            '0' ..= '9' => tokenize_number_or_float(slice_to_end),

            // For other keywords (and true and false), instead go until next whitespace and match
            _ => {
                let kw = preprocess_keyword_or_varname(slice_to_end);
                match kw {
                    "var" => (Var, 3),
                    "func" => (Func, 4),
                    "for" => (For, 3),
                    "while" => (While, 5),
                    "print" => (Print, 5),
                    "return" => (Return, 6),
                    "if" => (If, 2),
                    "true" => (Boolean(true), 4),
                    "false" => (Boolean(false), 5),
                    "null" => (Null, 4),
                    // if everything else fails we just assume varname
                    _ => (VarName(kw.to_owned()), kw.len())
                }
            }
        };

        return (found_token, trim_start_comments(&slice_to_end[length..]));
    }

    pub fn full_scan(input: &str) -> Vec<Self> {
        let mut remaining = trim_start_comments(input);
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
