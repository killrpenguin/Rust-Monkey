#[allow(dead_code, unused)]
pub mod monkey_lexer {
    use crate::token::tokens::*;

    pub struct Lexer<'a> {
        pub input: &'a str,
        pos: usize,
        read_pos: usize,
        ch: u8,
    }
    impl<'a> Lexer<'a> {
        pub fn new(input: &'a str) -> Self {
            let mut lexer: Lexer = Lexer {
                input,
                pos: 0,
                read_pos: 0,
                ch: 0,
            };
            assert!(!lexer.input.is_empty());
            lexer.read_char();
            lexer
        }
    }

    impl<'a> Lexer<'a> {
        fn look_ahead(&mut self, num: Option<u8>) {
            let num:u8 = num.unwrap_or(0);
            
        }
        fn read_char(&mut self) {
            if self.read_pos >= self.input.len() {
                self.ch = 0;
            } else {
                self.ch = self.input.as_bytes()[self.read_pos];
            }
            self.pos = self.read_pos;
            self.read_pos += 1
        }
        
        fn read_identifier(&mut self) -> Option<&str> {
            let pos: usize = self.pos;
            loop {
                if !self.is_letter(self.ch) && !self.is_digit(self.ch) {
                    break;
                }
                self.read_char();
            }
            Some(&self.input[pos..self.pos])
        }
        
        fn is_letter(&mut self, ch: u8) -> bool {
            if ch.is_ascii_alphabetic() || ch == b'_' {
                return true;
            }
            false
        }
        
        fn is_digit(&mut self, ch: u8) -> bool {
            if ch.is_ascii_digit() || ch == b'.' {
                return true;
            }
            false
        }
        
        fn read_number(&mut self) -> Option<&str> {
            let pos: usize = self.pos;
            loop {
                if !self.is_digit(self.ch) {
                    break;
                }
                self.read_char();
            }
            Some(&self.input[pos..self.pos])
        }
        
        fn eat_whitespace(&mut self) {
            while self.ch.is_ascii_whitespace() {
                self.read_char();
            }
            assert!(!self.ch.is_ascii_whitespace())
        }
        
        fn peek_char(&mut self) -> u8 {
            if self.read_pos >= self.input.len() {
                 0
            } else {
                self.input.as_bytes()[self.read_pos]
            }
        }
        
        fn look_up_ident(&mut self, ident: &str) -> Option<Token> {
            let token: Option<Token> = match ident {
                "let" | "Let" => Some(Token::Let),
                "func" | "Func" => Some(Token::Func),
                "If" | "if" => Some(Token::If),
                "Else" | "else" => Some(Token::Else),
                "For" | "for" => Some(Token::For),
                "Return" | "return" => Some(Token::Ret),
                "True" | "true" => Some(Token::True(true)),
                "False" | "false" => Some(Token::False(false)),
                _ => None,
            };
            token
        }

        pub fn next_token(&mut self) -> Token {
            self.eat_whitespace();
            let tkn: Token = match self.ch {
                b'+' => Token::Plus,
                b'-' => Token::Minus,
                b'*' => Token::Asterisk,
                b'/' => Token::Slash,
                b',' => Token::Comma,
                b':' => Token::Colon,
                b';' => Token::Semicolon,
                b'(' => Token::Lparen,
                b')' => Token::Rparen,
                b'{' => Token::Lcurly,
                b'}' => Token::Rcurly,
                b'[' => Token::Lbrac,
                b']' => Token::Rbrac,
                b'=' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Token::Eq
                    }
                    _ => Token::Assign,
                },
                b'!' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Token::NotEq
                    }
                    _ => Token::Bang,
                },
                b'<' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Token::LtEq
                    }
                    _ => Token::Lt,
                },
                b'>' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Token::GtEq
                    }
                    _ => Token::Gt,
                },
                _ if self.ch.is_ascii_alphanumeric() => {
                    if self.is_letter(self.ch) {
                        let literal: String = self.read_identifier().unwrap().to_owned();
                        return self
                            .look_up_ident(&literal)
                            .unwrap_or(Token::Ident(literal.to_string()));
                    } else if self.is_digit(self.ch) {
                        let literal: String = self.read_identifier().unwrap().to_owned();
                        if literal.contains('.') {
                            return Token::Float(literal.parse().unwrap());
                        } else {
                            return Token::Int(literal.parse().unwrap());
                        }
                    } else {
                        return Token::Illegal;
                    }
                }
                0 => Token::Eof,
                _ => Token::Illegal,
            };
            self.read_char();
            tkn
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::monkey_lexer::*;
    use crate::token::tokens::*;

    #[test]
    fn test_complete_code() {
        let input: &str = r#"let five = 5.5;
let ten = 10;
let add = func(x, y) {x + y;};
let result = add(five, ten);
!-/*5;
5 < 10 > 5.5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 = 9;
<=
>=
10 != 3;"#;

        let tests: Vec<Token> = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Float(5.5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Func,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lcurly,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rcurly,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Float(5.5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lcurly,
            Token::Ret,
            Token::True(true),
            Token::Semicolon,
            Token::Rcurly,
            Token::Else,
            Token::Lcurly,
            Token::Ret,
            Token::False(false),
            Token::Semicolon,
            Token::Rcurly,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::Assign,
            Token::Int(9),
            Token::Semicolon,
            Token::LtEq,
            Token::GtEq,
            Token::Int(10),
            Token::NotEq,
            Token::Int(3),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input);

        for expect in tests {
            let tkn = lexer.next_token();

            assert_eq!(expect, tkn);
        }
    }
}
