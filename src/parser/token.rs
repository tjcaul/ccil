trait TokenHelper {
    fn trim_start_comments(self) -> Self;
    fn tokenize_string(self) -> (Token, usize);
    fn tokenize_number_or_float(self) -> (Token, usize);
    fn preprocess_until_whitespace_or_semicolon(self) -> Self;
}

impl TokenHelper for &str {
    /// Helper function to remove comments along with whitespace
    fn trim_start_comments(self) -> Self {
        let mut trimming = self.trim_start();
        while trimming.len() > 1 && &trimming[0..2] == "//" {
            // get rid of comments, up till next newline
            trimming = &trimming[trimming.find("\n").unwrap_or(trimming.len())..];
            // get rid of whitespace (incl the newline)
            trimming = trimming.trim_start();
        }
        return trimming;
    }

    /// Helper function to tokenize string literal
    fn tokenize_string(self) -> (Token, usize) {
        let closing_index = match self.find("\"") {
            Some(val) => val,
            None => panic!("No closing quote found when parsing string literal")
        };

        // Need to account for two quotation marks in size
        return (Token::String(self[0..closing_index].to_string()), closing_index + 2);
    }

    /// Helper function to tokenize numbers or floats, returning appropriate token type
    fn tokenize_number_or_float(self) -> (Token, usize) {
        let full_literal = self.preprocess_until_whitespace_or_semicolon();

        match full_literal.parse::<i32>() {
            Ok(val) => return (Token::Number(val), full_literal.len()),
            Err(_) => {}
        };

        match full_literal.parse::<f64>() {
            Ok(val) => return (Token::Float(val), full_literal.len()),
            Err(_) => panic!("Illegal number detected")
        }
    }

    /// Helper function to preprocess (not tokenize) whitespace or semicolon-separated info
    fn preprocess_until_whitespace_or_semicolon(self) -> Self {
        let closing_index = match self.find(|c: char| c.is_whitespace() || c == ';') {
            Some(val) => val,
            None => self.len()
        };

        return &self[0..closing_index];
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
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
    SingleAnd, SingleOr,

    // Comparison (single or double char)
    DoubleEqual, BangEqual,
    GreaterThan, GreaterThanEqual,
    LessThan, LessThanEqual,

    // Boolean
    Bang, And, Or,

    // Literal
    // We need 64 bits of float for guaranteed lossless conversion between num and float
    String(String), Number(i32), Float(f64),

    // Keywords
    Var, Func, For, While, Print, Return, If, True, False, Null,

    // Misc
    EOF
}

#[allow(unused)]
impl Token {
    pub fn needs_value(self) -> bool {
        matches!(self, Token::String(_)) || matches!(self, Token::Number(_))
    }

    /// Returns a token from the remaining text, following grammar rules.
    /// Also returns a slice of the remaining string to process.
    /// Input should have leading whitespace removed,
    /// and output is guaranteed to have leading whitespace removed.
    fn scan_token(slice_to_end: &str) -> (Self, &str) {
        if slice_to_end.is_empty() {
            return (Token::EOF, "");
        }

        let (found_token, length) = match &slice_to_end[0..1] {
            // Unambiguous single-chars are the easiest; we simply match directly.
            "(" => (Token::LeftParen, 1),
            ")" => (Token::RightParen, 1),
            "{" => (Token::LeftCurly, 1),
            "}" => (Token::RightCurly, 1),
            "[" => (Token::LeftSquare, 1),
            "]" => (Token::RightSquare, 1),
            "," => (Token::Comma, 1),
            "." => (Token::Dot, 1),
            "-" => (Token::Minus, 1),
            "+" => (Token::Plus, 1),
            // Guaranteed to be fine since we treat comments as whitespace
            "/" => (Token::Slash, 1),
            "*" => (Token::Star, 1),
            ";" => (Token::Semicolon, 1),

            // These leading tokens could vary in meaning based on second token.
            "=" => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::DoubleEqual, 2)
                } else {
                    (Token::Equal, 1)
                }
            }
            "!" => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::BangEqual, 2)
                } else {
                    (Token::Bang, 1)
                }
            }
            "<" => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::GreaterThanEqual, 2)
                } else {
                    (Token::GreaterThan, 1)
                }
            }
            ">" => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "=" {
                    (Token::LessThanEqual, 2)
                } else {
                    (Token::LessThan, 1)
                }
            }
            "&" => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "&" {
                    (Token::And, 2)
                } else {
                    (Token::SingleAnd, 1)
                }
            }
            "|" => {
                if slice_to_end.len() > 1 && &slice_to_end[1..2] == "|" {
                    (Token::Or, 2)
                } else {
                    (Token::SingleOr, 1)
                }
            }

            // Literals can be done via some matching trickery
            // For strings, just detect opening quote and delegate to function from there
            "\"" => slice_to_end[1..].tokenize_string(),

            // For numbers and floats, detect starting digit
            // Yes this is the best way
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => slice_to_end.tokenize_number_or_float(),

            // For other keywords, instead go until next whitespace and match
            _ => match slice_to_end.preprocess_until_whitespace_or_semicolon() {
                "var" => (Token::Var, 3),
                "func" => (Token::Func, 4),
                "for" => (Token::For, 3),
                "while" => (Token::While, 5),
                "print" => (Token::Print, 5),
                "return" => (Token::Return, 6),
                "if" => (Token::If, 2),
                "true" => (Token::True, 4),
                "false" => (Token::False, 5),
                "null" => (Token::Null, 4),
                _ => panic!("Unknown keyword found while tokenizing")
            }
        };

        return (found_token, &slice_to_end[length..].trim_start_comments());
    }

    pub fn full_scan(input: &str) -> Vec<Self> {
        let mut remaining = input;
        let mut retval = Vec::<Token>::new();
        while retval.last().unwrap_or(&Token::LeftParen) != &Token::EOF {
            let (next_token, unused_slice) = Token::scan_token(remaining);
            retval.push(next_token);
            remaining = unused_slice;
        }
        return retval;
    }
}
