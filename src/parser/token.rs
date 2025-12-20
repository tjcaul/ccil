#[allow(unused)]
pub enum Token {
    // Single-char
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma,
    Dot,
    Minus, Plus, Slash, Star,
    Semicolon,
    Equal,

    // Comparison (single or double char)
    DoubleEqual, BangEqual,
    GreaterThan, GreaterThanEqual,
    LessThan, LessThanEqual,

    // Boolean
    Bang, And, Or,

    // Literal
    String(String), Number(i64), True, False, Null,

    // Keywords
    Var, Func, For, While, Print, Return, If,

    // Misc
    Error, EOF
}

#[allow(unused)]
impl Token {
    fn needs_value(self) -> bool {
        return matches!(self, Token::String(_)) || matches!(self, Token::Number(_))
    }
}
