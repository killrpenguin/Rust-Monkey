#[allow(dead_code, unused)]
pub mod monkey_ast {
    use crate::token::tokens::*;
    use std::fmt;

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
    pub struct Ident(pub String);
    impl fmt::Display for Ident {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Ident({})", &self.0)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Stmt {
        Let(Expression, Expression),
        Return(Expression),
        ExprStmt(Expression, Option<Expression>),
    }
    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                Stmt::Let(expr1, expr2) => write!(f, "Let(\n  {}\n  {})\n", expr1, expr2),
                Stmt::Return(expr1) => write!(f, "Return(\n  {})\n", expr1),
                Stmt::ExprStmt(expr1, expr2) => {
                    write!(f, "(\n  {}\n  {})\n", expr1, expr2.as_ref().unwrap())
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Expression {
        Prefix(PrefixOp, Box<Expression>),
        Infix(Operator, Box<Expression>),
        ExprIdent(Ident),
        Literal(Type),
    }
    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                Expression::Prefix(op, expr) => write!(f, "Prefix(\n  {}\n  {})\n", op, expr),
                Expression::Infix(op, expr) => write!(f, "Infix(\n  {}\n  {})\n", op, expr),
                Expression::Literal(lit) => write!(f, "Literal(\n  {})\n", lit),
                Expression::ExprIdent(ident) => write!(f, "Ident(\n  {})\n", ident),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Int(usize),
        Float(f64),
        Bool(bool),
    }
    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                Type::Int(val) => write!(f, "Type: int\n  Value: {}", val),
                Type::Float(val) => write!(f, "Type: float\n  Value: {}", val),
                Type::Bool(val) => write!(f, "Type: bool\n  Value: {}", val),
            }
        }
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
            match &self {
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
    pub enum PrefixOp {
        Positive,
        Negative,
        NotEq,
    }
    impl fmt::Display for PrefixOp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                PrefixOp::Positive => write!(f, "Positive."),
                PrefixOp::Negative => write!(f, "Negative."),
                PrefixOp::NotEq => write!(f, "Not Equal."),
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
