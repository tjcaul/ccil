use crate::parser::{Parser, rules::ParseRule, token::Token};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Empty,
    Unary(Token, Box<Expr>),
    Binary(Token, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    CurlyGrouping(Box<Expr>),
    SquareGrouping(Box<Expr>),
    Literal(Token),
    CommaSeparatedList(Vec<Box<Expr>>),
    Subexprs(Vec<Box<Expr>>),
    Variable(Token),
    VaribleDeclaration(Box<Expr>),
    FunctionDeclaration(Box<Expr>, Box<Expr>, Box<Expr>),
    ForLoop(Box<Expr>, Box<Expr>),
    ForLoopArgs(Box<Expr>, Box<Expr>, Box<Expr>),
    WhileLoop(Box<Expr>, Box<Expr>),
    PrintStatement(Box<Expr>),
    ReturnStatement(Box<Expr>),
    IfStatement(Box<Expr>, Box<Expr>),
}

impl Parser {
    /// Internal function meant to be called at the start of an expected new child expression.
    /// We should parse recursively with expressions -> parse rules -> expressions, etc.
    /// Returns the resultant expression.
    fn generate_expression(&mut self) -> Expr {
        let current_token: Token = self.consume_and_return();
        if current_token == Token::EOF {
            return Expr::Empty;
        }

        let parse_rule = match ParseRule::get_parse_rule(&current_token) {
            Some(val) => val,
            None => self.raise_parsing_error(format!("Unexpected token {:?}", current_token))
        };

        return (parse_rule.handler)(self, &current_token);
    }

    /// Generate an expression with an expected type. Returns the expression.
    fn expected_expression(&mut self, expected: &Expr) -> Expr {
        let expr = self.generate_expression();
        if std::mem::discriminant(&expr) != std::mem::discriminant(expected) {
            self.raise_parsing_error(format!("Expected expression {:?}, generated expression {:?}", expected, expr))
        }
        return expr;
    }

    /// Generate a Subexpr type up until the next RightCurly token.
    fn generate_subexprs(&mut self) -> Expr {
        todo!()
    }

    /// Parse a unary expression (an operator followed by another expression)
    pub fn unary(&mut self, token: &Token) -> Expr {
        let expr = self.generate_expression();
        return Expr::Unary(token.clone(), Box::new(expr));
    }

    /// Parse a binary expression (an expression followed by an operator followed by another expression)
    pub fn binary(&mut self, token: &Token) -> Expr {
        let left_expr = match self.floating_expressions.pop() {
            Some(val) => val,
            None => self.raise_parsing_error(format!("Binary operator {:?} has no left hand side", token))
        };
        let right_expr = self.generate_expression();
        return Expr::Binary(token.clone(), Box::new(left_expr), Box::new(right_expr));
    }

    /// Parse a grouping expression (i.e. items grouped together with parentheses)
    pub fn grouping(&mut self, token: &Token) -> Expr {
        let opposite = match token {
            Token::LeftParen => Token::RightParen,
            Token::LeftCurly => Token::RightCurly,
            Token::LeftSquare => Token::RightSquare,
            _ => Token::Dummy // unreachable
        };

        while self.tokens_to_process.last().unwrap_or(&opposite) != &opposite {
            let expr = self.generate_expression();
            self.floating_expressions.push(expr);
        }

        self.consume_expected(opposite);
        
        let resultant_expr = self.floating_expressions.pop().unwrap_or(Expr::Empty);
        
        return match token {
            Token::LeftParen => Expr::Grouping(Box::new(resultant_expr)),
            Token::LeftCurly => Expr::CurlyGrouping(Box::new(resultant_expr)),
            Token::LeftSquare => Expr::SquareGrouping(Box::new(resultant_expr)),
            _ => Expr::Empty
        };
    }

    /// Parse a literal expression (i.e. a literal value)
    pub fn literal(&mut self, token: &Token) -> Expr {
        return Expr::Literal(token.clone());
    }

    /// Special parse handler for ambiguous token "-"
    pub fn minus(&mut self, token: &Token) -> Expr {
        if self.floating_expressions.last().is_some() {
            return self.binary(token);
        }
        return self.unary(token);
    }

    /// Fully parse the elements of a comma separated list.
    pub fn comma_separated_list(&mut self, _token: &Token) -> Expr {
        let left_expr = match self.floating_expressions.pop() {
            Some(val) => val,
            None => self.raise_parsing_error("Unexpected comma".to_string())
        };
        let mut expr_list = vec![Box::new(left_expr)];
        loop {
            let rhs = self.generate_expression();
            expr_list.push(Box::new(rhs));
            if self.tokens_to_process.last().is_some_and(|x| x != &Token::Comma) {
                break;
            }
            self.consume_expected(Token::Comma);
        }

        return Expr::CommaSeparatedList(expr_list);
    }

    /// Used to produce assignments; essentially same as the binary function
    /// with an additional check to make sure the right hand side is a VarName.
    pub fn assignment(&mut self, token: &Token) -> Expr {
        let binary = self.binary(token);
        match binary {
            Expr::Binary(_, ref lhs, _) => {
                // if left hand side is not a variable expression
                if std::mem::discriminant(&**lhs) != std::mem::discriminant(&Expr::Variable(Token::Dummy)) {
                    self.raise_parsing_error("Illegal assignemt".to_string());
                }
            }
            _ => ()
        };
        return binary;
    }

    /// Return an expression containing a variable.
    pub fn variable(&mut self, token: &Token) -> Expr {
        return Expr::Variable(token.clone());
    }

    /// Parse an expression declaraing a variable, which essentially just contains a variable declaration.
    pub fn variable_declaration(&mut self, _token: &Token) -> Expr {
        let variable = self.expected_expression(
            &Expr::Variable(Token::Dummy)
        );
        self.floating_expressions.push(variable);
        let assignment = self.expected_expression(
            &Expr::Binary(Token::Dummy, Box::new(Expr::Empty), Box::new(Expr::Empty))
        );
        return Expr::VaribleDeclaration(Box::new(assignment));
    }

    /// Parse an expression declaraing a function, which contains (in order):
    /// The function name as a Variable, the arguments as a Grouping, and the body as a
    /// CurlyGrouping of a Subexprs.
    pub fn function_declaration(&mut self, _token: &Token) -> Expr {
        self.generate_subexprs();
        todo!()
    }

    /// Parse an expression declaring a for loop, which contains (in order):
    /// The C-style for loop arguments as a ForLoopArgs, and the body as a CurlyGrouping
    /// of a Subexprs.
    pub fn for_loop(&mut self, _token: &Token) -> Expr {
        todo!()
    }

    /// Parse an expression declaring a while loop, which contains (in order):
    /// The while loop condition, and the body as a CurlyGrouping of a Subexprs.
    pub fn while_loop(&mut self, _token: &Token) -> Expr {
        todo!()
    }

    /// Parse a print statement, with its only field being its argument.
    pub fn print_statement(&mut self, _token: &Token) -> Expr {
        self.consume_expected(Token::LeftParen);
        let argument = self.generate_expression();
        self.consume_expected(Token::RightParen);
        return Expr::PrintStatement(Box::new(argument));
    }

    /// Parse a return statement, with its only field being the return value.
    pub fn return_statement(&mut self, _token: &Token) -> Expr {
        Expr::ReturnStatement(Box::new(self.generate_expression()))
    }

    /// Parse an if statement, which contains (in order):
    /// The condition, and the body as a CurlyGrouping of a Subexprs.
    pub fn if_statement(&mut self, _token: &Token) -> Expr {
        todo!()
    }
}
