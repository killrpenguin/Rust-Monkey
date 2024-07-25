#[allow(dead_code, unused)]
pub mod monkey_parser {
    use std::any::Any;
    use std::path::PrefixComponent;

    use crate::ast::monkey_ast::*;
    use crate::lexer::monkey_lexer::*;
    use crate::token::tokens::*;
    use std::{error, fmt};

    type Result<'a, T> = std::result::Result<T, Box<dyn error::Error>>;

    // std::any::type_name is unstable. Output may change.
    // No guarantee that all parts of a type will appear in returned string.
    macro_rules! get_fn_name {
        () => {{
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f").unwrap()
        }};
    }

    #[derive(Debug)]
    pub struct MonkeyError<T: AsRef<str> + fmt::Debug + fmt::Display> {
        err_in: T,
    }

    impl<T: AsRef<str> + std::fmt::Debug + fmt::Display> MonkeyError<T> {
        pub fn new(f_name: T) -> MonkeyError<T> {
            MonkeyError { err_in: f_name }
        }
    }

    impl<T: AsRef<str> + fmt::Debug + fmt::Display> error::Error for MonkeyError<T> {}

    impl<T: AsRef<str> + fmt::Debug + fmt::Display> fmt::Display for MonkeyError<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parser Error in {}.", self.err_in)
        }
    }

    #[derive(Debug)]
    pub enum ErrorType {
        InvalidToken(Box<(dyn error::Error + 'static)>, Option<Token>),
    }

    impl error::Error for ErrorType {}

    impl From<Box<dyn error::Error>> for ErrorType {
        fn from(error: Box<(dyn error::Error + 'static)>) -> ErrorType {
            ErrorType::InvalidToken(error, None)
        }
    }
    impl fmt::Display for ErrorType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorType::InvalidToken(err, tkn) => {
                    write!(f, "{} Token::{} is Invalid.", err, tkn.clone().unwrap())
                }
            }
        }
    }

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

        pub fn parser_err<T: AsRef<str> + fmt::Debug + fmt::Display + 'static>(
            &self,
            name: T,
        ) -> Box<MonkeyError<T>> {
            return Box::new(MonkeyError::new(name));
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
            &self.cur_token == token
        }

        pub fn next_token_is(&self, token: &Token) -> bool {
            &self.next_token == token
        }

        pub fn next_token_if_is(&mut self, token: Token) -> Option<()> {
            if self.next_token_is(&token) {
                self.next_token();
                Some(())
            } else {
                None
            }
        }

        pub fn is_operator(&mut self) -> bool {
            match self.cur_token {
                Token::Plus => true,
                Token::Minus => true,
                Token::Asterisk => true,
                Token::Slash => true,
                Token::Eq => true,
                Token::NotEq => true,
                Token::Lt => true,
                Token::Gt => true,
                Token::LtEq => true,
                Token::GtEq => true,
                _ => return false,
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
                program_stmts.push(self.parse_stmt());
                self.next_token()
            }
            program_stmts
        }

        pub fn parse_stmt(&mut self) -> Stmt {
            match self.cur_token {
                Token::Let => self.parse_let_stmt().unwrap(),
                Token::Ret => self.parse_ret_stmt().unwrap(),
                _ => self.parse_expr_stmt().unwrap(),
            }
        }

        pub fn parse_let_stmt(&mut self) -> Result<Stmt> {
            self.next_token();
            let ident = self.parse_ident()?;
            self.next_token_if_is(Token::Assign);
            self.next_token();
            let literal = self.parse_expr()?;
            self.next_token_if_is(Token::Semicolon);
            Ok(Stmt::Let(Expression::ExprIdent(ident), literal))
        }

        pub fn parse_ret_stmt(&mut self) -> Result<Stmt> {
            self.next_token();
            let ret_expr = self.parse_expr()?;
            self.next_token_if_is(Token::Semicolon);
            Ok(Stmt::Return(ret_expr))
        }

        pub fn parse_expr_stmt(&mut self) -> Result<Stmt> {
            let l_val = self.parse_expr()?;
            while self.is_operator() {
                if let Ok(infix) = self.parse_infix() {
                    self.next_token_if_is(Token::Semicolon);
                    return Ok(Stmt::ExprStmt(l_val, Some(infix)));
                }
            }
            self.next_token_if_is(Token::Semicolon);
            Ok(Stmt::ExprStmt(l_val, None))
        }

        pub fn parse_infix(&mut self) -> Result<Expression> {
            let operator = self.parse_operator()?;
            self.next_token();
            let r_val = self.parse_expr()?;
            Ok(Expression::Infix(operator, Box::new(r_val)))
        }

        pub fn parse_prefix(&mut self) -> Result<Expression> {
            let prefix = match self.cur_token {
                Token::Minus => PrefixOp::Negative,
                Token::Bang => PrefixOp::NotEq,
                Token::Plus => PrefixOp::Positive,
                _ => return Err(Box::new(MonkeyError::new(get_fn_name!()))),
            };
            self.next_token();
            let expr = self.parse_expr()?;
            Ok(Expression::Prefix(prefix, Box::new(expr)))
        }

        pub fn parse_expr(&mut self) -> Result<Expression> {
            let l_val = match self.cur_token {
                Token::Int(_) => Ok(Expression::Literal(self.parse_int()?)),
                Token::Float(_) => Ok(Expression::Literal(self.parse_float()?)),
                Token::True(_) | Token::False(_) => Ok(Expression::Literal(self.parse_bool()?)),
                Token::Ident(_) => Ok(Expression::ExprIdent(self.parse_ident()?)),
                Token::Plus | Token::Minus | Token::Bang => Ok(self.parse_prefix()?),
                _ => {
                    return Err(Box::new(ErrorType::InvalidToken(
                        Box::new(MonkeyError::new(get_fn_name!())),
                        Some(self.cur_token.clone()),
                    )))
                }
            };

            self.next_token();
            l_val
        }

        pub fn parse_operator(&mut self) -> Result<Operator> {
            match self.cur_token {
                Token::Plus => Ok(Operator::Plus),
                Token::Minus => Ok(Operator::Minus),
                Token::Asterisk => Ok(Operator::Multiply),
                Token::Slash => Ok(Operator::Divide),
                Token::Eq => Ok(Operator::Eq),
                Token::NotEq => Ok(Operator::NotEq),
                Token::Lt => Ok(Operator::Lt),
                Token::Gt => Ok(Operator::Gt),
                Token::LtEq => Ok(Operator::LtEq),
                Token::GtEq => Ok(Operator::GtEq),
                _ => return Err(Box::new(MonkeyError::new(get_fn_name!()))),
            }
        }

        pub fn parse_ident(&mut self) -> Result<Ident> {
            match self.cur_token {
                Token::Ident(ref ident) => Ok(Ident(ident.to_string())),
                _ => Err(Box::new(MonkeyError::new(get_fn_name!()))),
            }
        }

        pub fn parse_float(&mut self) -> Result<Type> {
            match self.cur_token {
                Token::Float(num) => Ok(Type::Float(num)),
                _ => Err(Box::new(MonkeyError::new(get_fn_name!()))),
            }
        }

        pub fn parse_bool(&mut self) -> Result<Type> {
            match self.cur_token {
                Token::True(val) | Token::False(val) => Ok(Type::Bool(val)),
                _ => Err(Box::new(MonkeyError::new(get_fn_name!()))),
            }
        }

        pub fn parse_int(&mut self) -> Result<Type> {
            match self.cur_token {
                Token::Int(num) => Ok(Type::Int(num)),
                _ => Err(Box::new(MonkeyError::new(get_fn_name!()))),
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
    fn test_parse_prefix() {
        let tests = vec![
            (
                "-5",
                Stmt::ExprStmt(
                    Expression::Prefix(
                        PrefixOp::Negative,
                        Box::new(Expression::Literal(Type::Int(5))),
                    ),
                    None,
                ),
            ),
            (
                "+3",
                Stmt::ExprStmt(
                    Expression::Prefix(
                        PrefixOp::Positive,
                        Box::new(Expression::Literal(Type::Int(3))),
                    ),
                    None,
                ),
            ),
            (
                "!1",
                Stmt::ExprStmt(
                    Expression::Prefix(
                        PrefixOp::NotEq,
                        Box::new(Expression::Literal(Type::Int(1))),
                    ),
                    None,
                ),
            ),
        ];
        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse();
            assert_eq!(vec![expect], program);
        }
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
                "x + 5;",
                Stmt::ExprStmt(
                    Expression::ExprIdent(Ident("x".to_string())),
                    Some(Expression::Infix(
                        Operator::Plus,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "x + x;",
                Stmt::ExprStmt(
                    Expression::ExprIdent(Ident("x".to_string())),
                    Some(Expression::Infix(
                        Operator::Plus,
                        Box::new(Expression::ExprIdent(Ident("x".to_string()))),
                    )),
                ),
            ),
            (
                "1 + 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Plus,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 - 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Minus,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 * 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Multiply,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 / 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Divide,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 > 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Gt,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 < 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Lt,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 == 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::Eq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 != 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::NotEq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 >= 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::GtEq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
            ),
            (
                "1 <= 5;",
                Stmt::ExprStmt(
                    Expression::Literal(Type::Int(1)),
                    Some(Expression::Infix(
                        Operator::LtEq,
                        Box::new(Expression::Literal(Type::Int(5))),
                    )),
                ),
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
    #[test]
    fn test_err_handling() {
        let tests = vec![
            ("!", "Parser Error in monkey::parser::monkey_parser::Parser::parse_expr. Token::Eof is Invalid."),
        ];
        for (input, expect) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            if let Err(err) = parser.parse_expr() {
                assert_eq!(expect, err.to_string());
            };
        }
    }
}
