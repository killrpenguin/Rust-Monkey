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
            return parser;
        }

        pub fn next_token(&mut self) {
            self.cur_token = self.next_token.clone();
            self.next_token = self.lexer.next_token();
        }

        pub fn cur_token_is(&self, token: Token) -> bool {
            self.cur_token == token
        }

        pub fn next_token_is(&self, token: &Token) -> bool {
            self.next_token == *token
        }

        pub fn expected_peek_is(&mut self, token: Token) -> bool {
            if self.next_token_is(&token) {
                self.next_token();
                return true;
            } else {
                return false;
            }
        }

        pub fn cur_precedence(&self) -> Precedence {
            Self::token_precedence(&self.cur_token)
        }

        pub fn next_precedence(&self) -> Precedence {
            Self::token_precedence(&self.next_token)
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

        pub fn parse(&mut self) -> Vec<Stmt> {
            let mut program_stmts: Vec<Stmt> = vec![];

            while !self.next_token_is(&Token::Eof) {
                match self.parse_stmt() {
                    Some(stmt) => {
                        program_stmts.push(stmt);
                    }
                    _ => {}
                }
                self.next_token()
            }
            program_stmts
        }

        pub fn parse_stmt(&mut self) -> Option<Stmt> {
            match self.cur_token {
                Token::Let => {
                    self.next_token();
                    self.parse_let_stmt()
                }
                Token::Ret => {
                    self.next_token();
                    self.parse_ret_stmt()
                }
                _ => Some(Stmt::ExprStmt(Box::new(
                    self.parse_expr()
                        .expect("Failed to parse expr in parse_stmt."),
                ))),
            }
        }

        pub fn parse_ret_stmt(&mut self) -> Option<Stmt> {
            self.next_token();
            todo!()
        }

        pub fn parse_let_stmt(&mut self) -> Option<Stmt> {
            let ident = match &self.cur_token {
                Token::Ident(ident) => self
                    .parse_ident()
                    .expect("Failed to unwrap ident in let stmt."),
                _ => return None,
            };
            match self.cur_token {
                Token::Assign => self.next_token(),
                _ => return None,
            };
            let lit = self
                .parse_expr()
                .expect("Failed to unwrap expr in let stmt.");

            Some(Stmt::Let(Expr::ExprIdent(ident), Box::new(lit)))
        }

        pub fn parse_expr(&mut self) -> Option<Expr> {
            let l_val = match self.cur_token {
                Token::Int(_) => self.parse_int().expect("Failed to parse int"),
                Token::Float(_) => self.parse_float().expect("Failed to parse float"),
                Token::True(_) | Token::False(_) => {
                    self.parse_bool().expect("Failed to parse bool")
                }
                _ => return None,
            };
            self.next_token();
            Some(Expr::Literal(l_val))
        }

        pub fn parse_ident(&mut self) -> Option<Ident> {
            let ret_val = match self.cur_token {
                Token::Ident(ref ident) => Some(Ident(ident.to_string())),
                _ => None,
            };
            self.next_token();
            ret_val
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
    fn test_one() {
        let input = r#"
let x = 5; "#;
        let mut parser = Parser::new(Lexer::new(input));
        let prog = parser.parse();

        assert_eq!(
            vec![Stmt::Let(
                Expr::ExprIdent(Ident("x".to_string())),
                Box::new(Expr::Literal(Type::Int(5)))
            )],
            prog
        );
    }

    #[test]
    fn test_let_stmt() {
        let input = r#"
    let x = 5;
    let y = 10;
    let foobar = 838383;
            "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse();
        assert_eq!(
            vec![
                Stmt::Let(
                    Expr::ExprIdent(Ident("x".to_string())),
                    Box::new(Expr::Literal(Type::Int(5)))
                ),
                Stmt::Let(
                    Expr::ExprIdent(Ident("y".to_string())),
                    Box::new(Expr::Literal(Type::Int(10)))
                ),
                Stmt::Let(
                    Expr::ExprIdent(Ident("foobar".to_string())),
                    Box::new(Expr::Literal(Type::Int(838383)))
                ),
            ],
            program
        );
    }
}
