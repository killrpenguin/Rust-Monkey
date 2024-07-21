#[allow(dead_code, unused)]
pub mod monkey_parser {
    use std::any::Any;
    use std::path::PrefixComponent;

    use crate::ast::monkey_ast::*;
    use crate::lexer::monkey_lexer::*;
    use crate::token::tokens::*;

    pub struct Parser<'a> {
        lexer: Lexer<'a>,
        cur_token: Token,
        next_token: Token,
    }

    impl<'a> Parser<'a> {
        pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
            let mut parser = Parser {
                lexer,
                cur_token: Token::Eof,
                next_token: Token::Eof,
            };
            parser.next_token();
            parser.next_token();
            parser
        }

        pub fn token_precedence(token: &Token) -> Precedence {
            match &token {
                Token::NotEq | Token::Eq => Precedence::Equals,
                Token::Lt | Token::LtEq => Precedence::Lessgreater,
                Token::Gt | Token::GtEq => Precedence::Lessgreater,
                Token::Asterisk | Token::Slash => Precedence::Product,
                Token::Plus | Token::Minus => Precedence::Sum,
                _ => Precedence::Lowest,
            }
        }

        pub fn next_token(&mut self) {
            self.cur_token = self.next_token.clone();
            self.next_token = self.lexer.next_token();
        }

        pub fn cur_token_is(&self, token: &Token) -> bool {
            self.cur_token == *token
        }

        pub fn next_token_is(&self, token: &Token) -> bool {
            self.next_token == *token
        }

        pub fn next_token_if_is(&mut self, token: Token) -> Option<()> {
            if self.next_token_is(&token) {
                self.next_token();
                Some(())
            } else {
                None
            }
        }

        pub fn cur_precedence(&self) -> Precedence {
            Self::token_precedence(&self.cur_token)
        }

        pub fn next_precedence(&self) -> Precedence {
            Self::token_precedence(&self.next_token)
        }

        pub fn parse(&mut self) -> Vec<Stmt> {
            let mut program_stmts: Vec<Stmt> = vec![];

            while !self.next_token_is(&Token::Eof) {
                if let Some(stmt) = self.parse_stmt() {
                    program_stmts.push(stmt)
                }
                self.next_token()
            }
            program_stmts
        }

        pub fn parse_stmt(&mut self) -> Option<Stmt> {
            match self.cur_token {
                Token::Let => self.parse_let_stmt(),
                Token::Ret => self.parse_ret_stmt(),
                _ => Some(
                    self.parse_expr_stmt()
                        .expect("Failed to parse recursive expression parser."),
                ),
            }
        }

        pub fn parse_let_stmt(&mut self) -> Option<Stmt> {
            if let Token::Ident(_) = &self.next_token {
                self.next_token();
            } else {
                return None;
            }
            let ident = match self.parse_ident() {
                Some(ident) => ident,
                None => return None,
            };
            self.next_token_if_is(Token::Assign);
            self.next_token();
            let lit = match self.parse_expr() {
                Some(expr) => expr,
                None => return None,
            };
            self.next_token_if_is(Token::Semicolon);
            Some(Stmt::Let(Expression::ExprIdent(ident), lit))
        }

        pub fn parse_ret_stmt(&mut self) -> Option<Stmt> {
            self.next_token();
            let ret_expr = match self.parse_expr() {
                Some(expr) => expr,
                None => return None,
            };
            self.next_token_if_is(Token::Semicolon);
            Some(Stmt::Return(ret_expr))
        }

        pub fn parse_expr_stmt(&mut self) -> Option<Stmt> {
            let l_val = self
                .parse_expr()
                .expect("Failed to parse l val in expr stmt.");
            if let Some(_) = self.parse_operator() {
                if let Some(infix) = self.parse_infix() {
                    return Some(Stmt::ExprStmt(l_val, Some(infix)));
                } else {
                    None
                }
            } else {
                self.next_token_if_is(Token::Semicolon);
                return Some(Stmt::ExprStmt(l_val, None))
            }
        }

        pub fn parse_infix(&mut self) -> Option<Expression> {
            let operator = match self.parse_operator() {
                Some(op) => op,
                None => return None,
            };
            self.next_token();
            let r_val = match self.parse_expr() {
                Some(val) => val,
                None => return None,
            };
            Some(Expression::Infix(operator, Box::new(r_val)))
        }

        pub fn parse_expr(&mut self) -> Option<Expression> {
            let l_val = match self.cur_token {
                Token::Int(_) => Some(Expression::Literal(
                    self.parse_int()
                        .expect("Failed to parse int in parse expr."),
                )),
                Token::Float(_) => Some(Expression::Literal(
                    self.parse_float()
                        .expect("Failed to parse float in parse expr."),
                )),
                Token::True(_) | Token::False(_) => Some(Expression::Literal(
                    self.parse_bool()
                        .expect("Failed to parse bool in parse expr."),
                )),

                Token::Ident(_) => Some(Expression::ExprIdent(
                    self.parse_ident()
                        .expect("Failed to parse ident in parse expr."),
                )),
                _ => return None,
            };
            if !self.next_token_is(&Token::Semicolon) {
                self.next_token();
            }
            l_val
        }

        pub fn parse_operator(&mut self) -> Option<Operator> {
            match self.cur_token {
                Token::Plus => Some(Operator::Plus),
                Token::Minus => Some(Operator::Minus),
                Token::Asterisk => Some(Operator::Multiply),
                Token::Slash => Some(Operator::Divide),
                Token::Eq => Some(Operator::Eq),
                Token::NotEq => Some(Operator::NotEq),
                Token::Lt => Some(Operator::Lt),
                Token::Gt => Some(Operator::Gt),
                Token::LtEq => Some(Operator::LtEq),
                Token::GtEq => Some(Operator::GtEq),
                _ => None,
            }
        }

        pub fn parse_ident(&mut self) -> Option<Ident> {
            match self.cur_token {
                Token::Ident(ref ident) => Some(Ident(ident.to_string())),
                _ => None,
            }
        }

        pub fn parse_float(&mut self) -> Option<Type> {
            match self.cur_token {
                Token::Float(num) => Some(Type::Float(num)),
                _ => None,
            }
        }

        pub fn parse_bool(&mut self) -> Option<Type> {
            match self.cur_token {
                Token::True(val) | Token::False(val) => Some(Type::Bool(val)),
                _ => None,
            }
        }

        pub fn parse_int(&mut self) -> Option<Type> {
            match self.cur_token {
                Token::Int(num) => Some(Type::Int(num)),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::monkey_ast::*;
    use crate::lexer::monkey_lexer::*;
    use crate::parser::monkey_parser::*;

    #[test]
    fn test_let_stmts() {
        let input = r#"
    let x = 5;
    let y = 10.1;
    let foobar = 838383;
            "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();
        assert_eq!(
            vec![
                Stmt::Let(
                    Expression::ExprIdent(Ident("x".to_string())),
                    Expression::Literal(Type::Int(5))
                ),
                Stmt::Let(
                    Expression::ExprIdent(Ident("y".to_string())),
                    Expression::Literal(Type::Float(10.1))
                ),
                Stmt::Let(
                    Expression::ExprIdent(Ident("foobar".to_string())),
                    Expression::Literal(Type::Int(838383))
                ),
            ],
            program
        );
    }
    #[test]
    fn test_return_stmts() {
        let input = r#"
    return 37;
    return x;
"#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();
        assert_eq!(
            vec![
                Stmt::Return(Expression::Literal(Type::Int(37))),
                Stmt::Return(Expression::ExprIdent(Ident("x".to_string()))),
            ],
            program
        );
    }
    #[test]
    fn test_expr_stmts() {
        let tests = vec![
            (
                "1 + 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Plus,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 - 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Minus,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 * 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Multiply,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 / 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Divide,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 > 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Gt,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 < 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Lt,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 == 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Eq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 != 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::NotEq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 >= 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::GtEq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "1 <= 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::LtEq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ))),
            ),
            (
                "3;",
                Stmt::ExprStmt(Expression::Literal(Type::Int(3)), None),
            ),
        ];
        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();
            assert_eq!(vec![expect], program);
        }
    }
}
