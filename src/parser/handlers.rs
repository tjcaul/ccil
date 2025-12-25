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
        if self.tokens_to_process.len() > 0 && self.tokens_to_process[0] == Token::RightParen {
            // empty parentheses
            return Expr::Grouping(Box::new(Expr::Empty));
        }
        let expr = self.expression();
        self.consume_expected(Token::RightParen);
        return Expr::Grouping(Box::new(expr));
    }
}