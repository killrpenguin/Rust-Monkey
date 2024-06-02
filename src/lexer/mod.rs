#[allow(dead_code, unused)]
pub mod monkey_lexer {
    use crate::token::tokens::*;

    pub struct Lexer<'a> {
        pub input: &'a String,
        pos: usize,
        read_pos: usize,
        ch: u8,
    }
    impl<'a> Lexer<'a> {
        pub fn new(input: &'a String) -> Self {
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

    pub trait L {
        fn read_char(&mut self);
        fn read_identifier(&mut self) -> Option<String>;
        fn is_letter(&mut self, ch: u8) -> bool;
        fn is_digit(&mut self, ch: u8) -> bool;
        fn read_number(&mut self) -> Option<String>;
        fn eat_whitespace(&mut self);
        fn peek_char(&mut self) -> u8;
        fn next_token(&mut self) -> TokenType;
        fn look_up_ident(&mut self, ident: &str) -> Option<TokenType>;
    }
    impl<'a> L for Lexer<'a> {
        fn read_char(&mut self) {
            if self.read_pos >= self.input.len() {
                self.ch = 0;
            } else {
                self.ch = self.input.as_bytes()[self.read_pos];
            }
            self.pos = self.read_pos;
            self.read_pos += 1
        }
        fn read_identifier(&mut self) -> Option<String> {
            let pos: usize = self.pos;
            loop {
                if !self.is_letter(self.ch) && !self.is_digit(self.ch) {
                    break;
                }
                self.read_char();
            }
            let ident: &str = &self.input[pos..self.pos];
            Some(String::from(ident))
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
        fn read_number(&mut self) -> Option<String> {
            let pos: usize = self.pos;
            loop {
                if !self.is_digit(self.ch) {
                    break;
                }
                self.read_char();
            }
            let ident: &str = &self.input[pos..self.pos];
            Some(String::from(ident))
        }
        fn eat_whitespace(&mut self) {
            while self.ch.is_ascii_whitespace() {
                self.read_char();
            }
        }
        fn peek_char(&mut self) -> u8 {
            if self.read_pos >= self.input.len() {
                 0
            } else {
                self.input.as_bytes()[self.read_pos]
            }
        }
        fn look_up_ident(&mut self, ident: &str) -> Option<TokenType> {
            let token: Option<TokenType> = match ident {
                "let" | "Let" => Some(TokenType::Let),
                "func" | "Func" => Some(TokenType::Func),
                "If" | "if" => Some(TokenType::If),
                "Else" | "else" => Some(TokenType::Else),
                "For" | "for" => Some(TokenType::For),
                "Return" | "return" => Some(TokenType::Ret),
                "True" | "true" => Some(TokenType::True(true)),
                "False" | "false" => Some(TokenType::False(false)),
                _ => None,
            };
            token
        }
        fn next_token(&mut self) -> TokenType {
            self.eat_whitespace();
            let tkn: TokenType = match self.ch {
                b'+' => TokenType::Plus,
                b'-' => TokenType::Minus,
                b'*' => TokenType::Asterisk,
                b'/' => TokenType::Slash,
                b',' => TokenType::Comma,
                b':' => TokenType::Colon,
                b';' => TokenType::Semicolon,
                b'(' => TokenType::Lparen,
                b')' => TokenType::Rparen,
                b'{' => TokenType::Lcurly,
                b'}' => TokenType::Rcurly,
                b'[' => TokenType::Lbrac,
                b']' => TokenType::Rbrac,
                b'=' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        TokenType::Eq
                    }
                    _ => TokenType::Assign,
                },
                b'!' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        TokenType::NotEq
                    }
                    _ => TokenType::Bang,
                },
                b'<' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        TokenType::LtEq
                    }
                    _ => TokenType::Lt,
                },
                b'>' => match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        TokenType::GtEq
                    }
                    _ => TokenType::Gt,
                },
                _ if self.ch.is_ascii_alphanumeric() => {
                    if self.is_letter(self.ch) {
                        let literal: String = self.read_identifier().unwrap();
                        return self
                            .look_up_ident(&literal)
                            .unwrap_or(TokenType::Ident(literal));
                    } else if self.is_digit(self.ch) {
                        let literal: String = self.read_identifier().unwrap();
                        if literal.contains('.') {
                            return TokenType::Float(literal.parse().unwrap());
                        } else {
                            return TokenType::Int(literal.parse().unwrap());
                        }
                    } else {
                        return TokenType::Illegal;
                    }
                }
                0 => TokenType::Eof,
                _ => TokenType::Illegal,
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
    fn test_int_or_float_i() {
        let input: String = String::from("5");
        let mut lexer = Lexer::new(&input);
        let expect: TokenType = TokenType::Int(5);
        assert_eq!(expect, lexer.next_token());
    }
    #[test]
    fn test_return() {
        let input: String = String::from(
            r#"
      return"#,
        );
        let mut lexer = Lexer::new(&input);
        let expect: TokenType = TokenType::Ret;
        assert_eq!(expect, lexer.next_token());
    }
    #[test]
    fn test_two_part_op() {
        let input: String = String::from(" 10 >= 10; ");
        let mut lexer = Lexer::new(&input);
        let tests: Vec<TokenType> = vec![
            TokenType::Int(10),
            TokenType::GtEq,
            TokenType::Int(10),
            TokenType::Semicolon,
        ];
        for expect in tests {
            let tkn = lexer.next_token();

            assert_eq!(expect, tkn);
        }
    }
    #[test]
    fn test_int_or_float_f() {
        let input: String = String::from("5.5");
        let mut lexer = Lexer::new(&input);
        let expect: TokenType = TokenType::Float(5.5);
        assert_eq!(expect, lexer.next_token());
    }
    #[test]
    fn test_illegal() {
        let input: String = String::from("@");
        let mut lexer = Lexer::new(&input);
        let expect: TokenType = TokenType::Illegal;
        assert_eq!(expect, lexer.next_token());
    }
    #[test]
    fn test_check_idents() {
        let input: Vec<&str> = vec!["Func", "let", "If", "else", "For", "return "];
        let tests: Vec<TokenType> = vec![
            TokenType::Func,
            TokenType::Let,
            TokenType::If,
            TokenType::Else,
            TokenType::For,
            TokenType::Ret,
        ];
        for (expect, input) in tests.iter().zip(input.iter()) {
            let inpt: String = input.to_string();
            let mut lexer = Lexer::new(&inpt);
            let ret_val: TokenType = lexer.next_token();
            assert_eq!(expect, &ret_val);
        }
    }
    #[test]
    fn test_peek_char_not_eq() {
        let input: String = "=!".to_string();
        let mut lexer = Lexer::new(&input);
        let expect: u8 = b'@';
        assert_ne!(expect, lexer.peek_char());
    }
    #[test]
    fn test_peek_char() {
        let input: String = "=!".to_string();
        let mut lexer = Lexer::new(&input);
        let expect: u8 = b'!';
        assert_eq!(expect, lexer.peek_char());
    }
    #[test]
    fn test_complete_code() {
        let input: String = String::from(
            r#"let five = 5.5;
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
10 != 3;"#,
        );
        let tests: Vec<TokenType> = vec![
            TokenType::Let,
            TokenType::Ident(String::from("five")),
            TokenType::Assign,
            TokenType::Float(5.5),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("ten")),
            TokenType::Assign,
            TokenType::Int(10),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("add")),
            TokenType::Assign,
            TokenType::Func,
            TokenType::Lparen,
            TokenType::Ident(String::from("x")),
            TokenType::Comma,
            TokenType::Ident(String::from("y")),
            TokenType::Rparen,
            TokenType::Lcurly,
            TokenType::Ident(String::from("x")),
            TokenType::Plus,
            TokenType::Ident(String::from("y")),
            TokenType::Semicolon,
            TokenType::Rcurly,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("result")),
            TokenType::Assign,
            TokenType::Ident(String::from("add")),
            TokenType::Lparen,
            TokenType::Ident(String::from("five")),
            TokenType::Comma,
            TokenType::Ident(String::from("ten")),
            TokenType::Rparen,
            TokenType::Semicolon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::Int(5),
            TokenType::Semicolon,
            TokenType::Int(5),
            TokenType::Lt,
            TokenType::Int(10),
            TokenType::Gt,
            TokenType::Float(5.5),
            TokenType::Semicolon,
            TokenType::If,
            TokenType::Lparen,
            TokenType::Int(5),
            TokenType::Lt,
            TokenType::Int(10),
            TokenType::Rparen,
            TokenType::Lcurly,
            TokenType::Ret,
            TokenType::True(true),
            TokenType::Semicolon,
            TokenType::Rcurly,
            TokenType::Else,
            TokenType::Lcurly,
            TokenType::Ret,
            TokenType::False(false),
            TokenType::Semicolon,
            TokenType::Rcurly,
            TokenType::Int(10),
            TokenType::Eq,
            TokenType::Int(10),
            TokenType::Semicolon,
            TokenType::Int(10),
            TokenType::Assign,
            TokenType::Int(9),
            TokenType::Semicolon,
            TokenType::LtEq,
            TokenType::GtEq,
            TokenType::Int(10),
            TokenType::NotEq,
            TokenType::Int(3),
            TokenType::Semicolon,
            TokenType::Eof,
        ];
        let mut lexer = Lexer::new(&input);

        for expect in tests {
            let tkn = lexer.next_token();

            assert_eq!(expect, tkn);
        }
    }
}
