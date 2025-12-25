use crate::parser::{Parser, expr::Expr, token::Token};

impl Parser {
    /// Internal function meant to be called at the start of an expected new expression.
    /// We should parse recursively with expressions -> parse steps, etc.
    /// Returns the resultant expression.
    fn expression(&mut self) -> Expr {
        todo!()
    }

    /// Parse a grouping expression (i.e. items grouped together with parentheses)
    pub fn grouping(&mut self) -> Expr {
        let expr;
        expr = self.expression();
        self.consume_expected(Token::RightParen);
        return Expr::Grouping(Box::new(expr));
    }
}