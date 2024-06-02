#[allow(dead_code, unused)]

pub mod tokens {
    use core::fmt;

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
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
    impl fmt::Display for TokenType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TokenType::Eof => write!(f, "Eof"),
                TokenType::Illegal => write!(f, "Illegal"),
                TokenType::Ident(string) => write!(f, "Ident"),
                TokenType::Int(usize) => write!(f, "Int"),
                TokenType::Float(f64) => write!(f, "Float"),
                TokenType::Assign => write!(f, "Assign"),
                TokenType::Plus => write!(f, "Plus"),
                TokenType::Minus => write!(f, "Minus"),
                TokenType::Bang => write!(f, "Bang"),
                TokenType::Asterisk => write!(f, "Asterisk"),
                TokenType::Slash => write!(f, "Slash"),
                TokenType::Comma => write!(f, "Comma"),
                TokenType::Eq => write!(f, "Eq"),
                TokenType::NotEq => write!(f, "NotEq"),
                TokenType::LtEq => write!(f, "LtEq"),
                TokenType::GtEq => write!(f, "GtEq"),
                TokenType::Lt => write!(f, "Lt"),
                TokenType::Gt => write!(f, "Gt"),
                TokenType::Colon => write!(f, "Colon"),
                TokenType::Semicolon => write!(f, "Semicolon"),
                TokenType::Lparen => write!(f, "Lparen"),
                TokenType::Rparen => write!(f, "Rparen"),
                TokenType::Lcurly => write!(f, "Lcurly"),
                TokenType::Rcurly => write!(f, "Rcurly"),
                TokenType::Lbrac => write!(f, "Lbrac"),
                TokenType::Rbrac => write!(f, "Rbrac"),
                TokenType::Func => write!(f, "Func"),
                TokenType::Let => write!(f, "Let"),
                TokenType::True(true) | TokenType::True(false) => write!(f, "True"),
                TokenType::False(false) | TokenType::False(true) => write!(f, "False"),
                TokenType::If => write!(f, "If"),
                TokenType::Else => write!(f, "Else"),
                TokenType::Ret => write!(f, "Ret"),
                TokenType::For => write!(f, "TokenType::For"),
            }        
        }
    }
}
