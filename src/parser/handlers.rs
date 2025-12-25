use crate::parser::{Parser, expr::Expr, token::Token};

impl Parser {
    fn expression(&mut self) -> Expr {
        todo!()
    }

    pub fn grouping(&mut self) -> Expr {
        let expr;
        expr = self.expression();
        self.parse_expect(Token::RightParen);
        return Expr::Grouping(Box::new(expr.clone()));
    }
}