use std::fmt;
use std::convert;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Const {
    Int(i128),
    Float(f64),
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Const::Int(i) => write!(f, "{}", i),
            Const::Float(i) => write!(f, "{}", i),
        }
    }
}

impl convert::From<i128> for Const {
    fn from(i: i128) -> Const {
        Const::Int(i)
    }
}

impl convert::From<f64> for Const {
    fn from(f: f64) -> Const {
        Const::Float(f)
    }
}

#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Null(Box<Expr>),
    Const(Const)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Add(lhs, rhs) => write!(f, "(+ {} {})", *lhs, *rhs),
            Expr::Sub(lhs, rhs) => write!(f, "(- {} {})", *lhs, *rhs),
            Expr::Mul(lhs, rhs) => write!(f, "(* {} {})", *lhs, *rhs),
            Expr::Div(lhs, rhs) => write!(f, "(/ {} {})", *lhs, *rhs),
            Expr::Neg(v) => write!(f, "(- {})", *v),
            Expr::Null(v) => write!(f, "(+ {})", *v),
            Expr::Const(c) => write!(f, "{}", c),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (Add(llhs, lrhs), Add(rlhs, rrhs)) => **llhs == **rlhs && **lrhs == **rrhs,
            (Sub(llhs, lrhs), Sub(rlhs, rrhs)) => **llhs == **rlhs && **lrhs == **rrhs,
            (Mul(llhs, lrhs), Mul(rlhs, rrhs)) => **llhs == **rlhs && **lrhs == **rrhs,
            (Div(llhs, lrhs), Div(rlhs, rrhs)) => **llhs == **rlhs && **lrhs == **rrhs,
            (Neg(lhs), Neg(rhs)) => **lhs == **rhs,
            (Null(lhs), Null(rhs)) => **lhs == **rhs,
            (Const(lhs), Const(rhs)) => lhs == rhs,
            (_, _) => false
        }
    }
}

impl Expr {
    pub fn new_binary(op: &str, lhs: Expr, rhs: Expr) -> Option<Expr> {
        match op {
            "+" => Some(Expr::Add(Box::from(lhs), Box::from(rhs))),
            "-" => Some(Expr::Sub(Box::from(lhs), Box::from(rhs))),
            "*" => Some(Expr::Mul(Box::from(lhs), Box::from(rhs))),
            "/" => Some(Expr::Div(Box::from(lhs), Box::from(rhs))),
            _ => None,
        }
    }

    pub fn new_unary(op: &str, expr: Expr) -> Option<Expr> {
        match op {
            "+" => Some(Expr::Null(Box::from(expr))),
            "-" => Some(Expr::Neg(Box::from(expr))),
            _ => None,
        }
    }    
}