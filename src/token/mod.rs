#[allow(dead_code, unused)]

pub mod tokens {
    use core::fmt;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        Illegal,
        Eof,
        // Identifiers,
        Ident(String),
        Int(usize),
        Float(f64),
        // Operators
        Assign,
        Plus,
        Minus,
        Bang,
        Asterisk,
        Slash,
        Comma,
        Eq,
        NotEq,
        LtEq,
        GtEq,
        Lt,
        Gt,
        // Delimiters
        Colon,
        Semicolon,
        Lparen,
        Rparen,
        Lcurly,
        Rcurly,
        Lbrac,
        Rbrac,
        // Keywords
        Func,
        Let,
        True(bool),
        False(bool),
        If,
        Else,
        Ret,
        For,
    }
    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Token::Eof => write!(f, "Eof"),
                Token::Illegal => write!(f, "Illegal"),
                Token::Ident(string) => write!(f, "Ident"),
                Token::Int(usize) => write!(f, "Int"),
                Token::Float(f64) => write!(f, "Float"),
                Token::Assign => write!(f, "Assign"),
                Token::Plus => write!(f, "Plus"),
                Token::Minus => write!(f, "Minus"),
                Token::Bang => write!(f, "Bang"),
                Token::Asterisk => write!(f, "Asterisk"),
                Token::Slash => write!(f, "Slash"),
                Token::Comma => write!(f, "Comma"),
                Token::Eq => write!(f, "Eq"),
                Token::NotEq => write!(f, "NotEq"),
                Token::LtEq => write!(f, "LtEq"),
                Token::GtEq => write!(f, "GtEq"),
                Token::Lt => write!(f, "Lt"),
                Token::Gt => write!(f, "Gt"),
                Token::Colon => write!(f, "Colon"),
                Token::Semicolon => write!(f, "Semicolon"),
                Token::Lparen => write!(f, "Lparen"),
                Token::Rparen => write!(f, "Rparen"),
                Token::Lcurly => write!(f, "Lcurly"),
                Token::Rcurly => write!(f, "Rcurly"),
                Token::Lbrac => write!(f, "Lbrac"),
                Token::Rbrac => write!(f, "Rbrac"),
                Token::Func => write!(f, "Func"),
                Token::Let => write!(f, "Let"),
                Token::True(true) | Token::True(false) => write!(f, "True"),
                Token::False(false) | Token::False(true) => write!(f, "False"),
                Token::If => write!(f, "If"),
                Token::Else => write!(f, "Else"),
                Token::Ret => write!(f, "Ret"),
                Token::For => write!(f, "TokenType::For"),
            }        
        }
    }
}
