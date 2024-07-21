#[allow(dead_code, unused)]
pub mod monkey_ast {
    use crate::token::tokens::*;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Ident(pub String);

    #[derive(Debug, Clone, PartialEq)]
    pub struct Program {
        statement_nodes: Vec<Stmt>,
    }
    impl Default for Program {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Program {
        pub fn new() -> Program {
            Program {
                statement_nodes: vec![],
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Stmt {
        Let(Expression, Expression),
        Return(Expression),
        ExprStmt(Expression, Option<Expression>),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Expression {
        Infix(Operator, Box<Expression>),
        ExprIdent(Ident),
        Literal(Type),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Int(usize),
        Float(f64),
        Bool(bool),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Operator {
        Plus,
        Minus,
        Multiply,
        Divide,
        Eq,
        NotEq,
        Lt,
        Gt,
        LtEq,
        GtEq,
    }

    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Operator::Plus => write!(f, "+"),
                Operator::Minus => write!(f, "-"),
                Operator::Multiply => write!(f, "*"),
                Operator::Divide => write!(f, "/"),
                Operator::Eq => write!(f, "="),
                Operator::NotEq => write!(f, "!="),
                Operator::Lt => write!(f, "<"),
                Operator::Gt => write!(f, ">"),
                Operator::LtEq => write!(f, "<="),
                Operator::GtEq => write!(f, ">="),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub enum Precedence {
        Lowest,
        Equals,
        Lessgreater,
        Sum,
        Product,
        Prefix,
        Call,
    }
}
