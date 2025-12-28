use crate::parser::{Parser, expr_compare::ExprType, rules::ParseRule, token::Token};

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
    FunctionCall(Token, Box<Expr>),
    ForLoop(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
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
    fn expected_expression(&mut self, expected: &ExprType) -> Expr {
        let expr = self.generate_expression();
        if !expr.is_type(expected) {
            self.raise_parsing_error(format!("Expected expression {:?}, generated expression {:?}", expected, expr))
        }
        return expr;
    }

    /// Generate a Subexpr type up until the next supplied token.
    fn generate_subexprs(&mut self, ending_token: &Token) -> Expr {
        let mut subparser = Parser {
            current_line: self.current_line,
            tokens_to_process: self.tokens_to_process.clone(),
            floating_expressions: Vec::new(),
            expressions: Vec::new()
        };
        while &subparser.peek() != ending_token {
            subparser.parse_step();
        }
        subparser.consume_expected(ending_token.clone());
        self.current_line = subparser.current_line;
        self.tokens_to_process = subparser.tokens_to_process;

        let mut boxed = Vec::<Box<Expr>>::new();
        for expression in subparser.expressions {
            boxed.push(Box::new(expression));
        }
        return Expr::Subexprs(boxed);
    }

    /// Generate a single expression up until the specified ending token.
    pub fn generate_until(&mut self, ending_token: Token) -> Expr {
        while self.peek() != ending_token && self.peek() != Token::EOF {
            let expr = self.generate_expression();
            self.floating_expressions.push(expr);
        }
        self.consume_expected(ending_token);
        return match self.floating_expressions.len() {
            0 => Expr::Empty,
            1 => self.floating_expressions.pop().unwrap(),
            _ => self.raise_parsing_error("Illegal expression".to_owned())
        };
    }

    /// Generate a semicolon-separated line.
    pub fn generate_until_semicolon(&mut self) -> Expr {
        self.generate_until(Token::Semicolon)
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
            if self.peek() != Token::Comma {
                break;
            }
            self.consume_expected(Token::Comma);
        }

        return Expr::CommaSeparatedList(expr_list);
    }

    /// Internal function to parse child CSLs used in function declaration/calls
    fn expect_grouped_csl(&mut self) -> Expr {
        let csl: Expr;
        self.consume_expected(Token::LeftParen);
        if self.peek() == Token::RightParen {
            // zero args
            csl = Expr::CommaSeparatedList(Vec::new());
        } else {
            let first_var = self.generate_expression();
            if self.peek() == Token::RightParen {
                // one arg
                csl = Expr::CommaSeparatedList(vec![Box::new(first_var)]);
            } else {
                // multiple args, defer to our default parser behaviour
                self.floating_expressions.push(first_var);
                csl = self.expected_expression(&ExprType::CommaSeparatedList);
            }
        }
        self.consume_expected(Token::RightParen);
        return csl;
    }

    /// Used to produce assignments; essentially same as the binary function
    /// with an additional check to make sure the right hand side is a VarName.
    pub fn assignment(&mut self, token: &Token) -> Expr {
        let binary = self.binary(token);
        match binary {
            Expr::Binary(_, ref lhs, _) => {
                // if left hand side is not a variable expression
                if !(&**lhs).is_type(&ExprType::Variable) {
                    self.raise_parsing_error("Illegal assignemt".to_string());
                }
            }
            _ => ()
        };
        return binary;
    }

    /// Return an expression containing a variable or function call.
    pub fn variable(&mut self, token: &Token) -> Expr {
        if self.peek() == Token::LeftParen {
            let args = self.expect_grouped_csl();
            return Expr::FunctionCall(token.clone(), Box::new(args));
        } else {
            return Expr::Variable(token.clone());
        }
    }

    /// Parse an expression declaraing a variable, which essentially just contains a variable declaration.
    pub fn variable_declaration(&mut self, _token: &Token) -> Expr {
        let variable = self.expected_expression(&ExprType::Variable);
        self.floating_expressions.push(variable);
        let assignment = self.expected_expression(
            &ExprType::Variable
        );
        return Expr::VaribleDeclaration(Box::new(assignment));
    }

    /// Parse an expression declaraing a function, which contains (in order):
    /// The function name as a Variable, the arguments as a CommaSeparatedList, and the body as a Subexprs.
    pub fn function_declaration(&mut self, _token: &Token) -> Expr {
        // Can't just use our builtin expression generator because it'll misparse as a function call
        // Luckily creating variables normally is super straightforward
        let variable = self.consume_expected(Token::VarName(String::new()));
        let var_expr = Expr::Variable(variable);

        let args= self.expect_grouped_csl();
        match &args {
            Expr::CommaSeparatedList(arg_list) => {
                for arg in arg_list {
                    if !(&**arg).is_type(&ExprType::Variable) {
                        self.raise_parsing_error(
                            "Non-variable found in function declaration args".to_owned()
                        )
                    }
                }
            },
            _ => () // unreachable
        }

        self.consume_expected(Token::LeftCurly);
        let subexprs = self.generate_subexprs(&Token::RightCurly);
        
        return Expr::FunctionDeclaration(Box::new(var_expr), Box::new(args), Box::new(subexprs))
    }

    /// Parse an expression declaring a for loop, which contains (in order):
    /// The C-style for loop arguments as the first three exprs,
    /// and the body as a Subexprs.
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
        let argument = self.generate_until(Token::RightParen);
        return Expr::PrintStatement(Box::new(argument));
    }

    /// Parse a return statement, with its only field being the return value.
    pub fn return_statement(&mut self, _token: &Token) -> Expr {
        // return captures everything, so we just take the rest and regenerate it
        let retval = self.generate_until_semicolon();
        // push a semicolon back onto stack to preserve invariant
        self.tokens_to_process.push(Token::Semicolon);
        return Expr::ReturnStatement(Box::new(retval));
    }

    /// Parse an if statement, which contains (in order):
    /// The condition, and the body as a CurlyGrouping of a Subexprs.
    pub fn if_statement(&mut self, _token: &Token) -> Expr {
        todo!()
    }
}
