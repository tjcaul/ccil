use crate::parser::{Parser, token::Token};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Unary(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
}

impl Parser {
    /// Internal function meant to be called at the start of an expected new expression.
    /// We should parse recursively with expressions -> parse steps -> expressions, etc.
    /// Returns the resultant expression.
    fn expression(&mut self) -> Expr {
        todo!()
    }

    /// Parse a unary expression (an operator followed by another expression)
    pub fn unary(&mut self, token: &Token) -> Expr {
        let expr = self.expression();
        return Expr::Unary(token.clone(), Box::new(expr));
    }

    /// Parse a binary expression (an expression followed by an operator followed by another expression)
    pub fn binary(&mut self, token: &Token) -> Expr {
        let left_expr = match self.expressions.pop() {
            Some(val) => val,
            None => self.raise_parsing_error(format!("Binary operator {:?} has no left hand side", token))
        };
        let right_expr = self.expression();
        return Expr::Binary(token.clone(), Box::new(left_expr), Box::new(right_expr));
    }

    /// Parse a grouping expression (i.e. items grouped together with parentheses)
    pub fn grouping(&mut self, _token: &Token) -> Expr {
        if self.tokens_to_process.len() > 0 && self.tokens_to_process[0] == Token::RightParen {
            // empty parentheses
            return Expr::Grouping(Box::new(Expr::Empty));
        }
        let expr = self.expression();
        self.consume_expected(Token::RightParen);
        return Expr::Grouping(Box::new(expr));
    }

    /// Parse a literal expression (i.e. a literal value)
    pub fn literal(&mut self, token: &Token) -> Expr {
        return Expr::Literal(token.clone());
    }
}
