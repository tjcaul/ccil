use crate::parser::{Parser, expr::Expr, token::Token};

impl Parser {
    fn expression(self) -> (Expr, Self) {
        todo!()
    }

    pub fn grouping(mut self) -> (Expr, Self) {
        let expr;
        (expr, self) = self.expression();
        self = self.parse_expect(Token::RightParen).unwrap();
        return (Expr::Grouping(Box::new(expr.clone())), self);
    }
}