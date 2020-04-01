use std::fmt;
use std::cmp::Ordering;
use super::rat;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Const {
    Int(i128),
    Float(f64),
    Rat(rat::Rat),
    Inf,
    NegInf,
    Undef
}

impl PartialOrd for Const {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Const::*;
        use Ordering::*;
        match (self, other) {
            (Undef, Undef) => Some(Equal),
            (Undef, _) => None,
            (_, Undef) => None,

            (Inf, Inf) => Some(Equal),
            (NegInf, NegInf) => Some(Equal),

            (Inf, _) => Some(Greater),
            (_, Inf) => Some(Less),
            (NegInf, _) => Some(Less),
            (_, NegInf) => Some(Greater),

            (Int(x), Int(y)) => Some(x.cmp(y)),
            (Int(x), Rat(y)) => Some(rat::Rat::from(*x).cmp(y)),
            (Int(x), Float(y)) => (*x as f64).partial_cmp(y),
            (Rat(x), Int(y)) => Some(x.cmp(&rat::Rat::from(*y))),
            (Float(x), Int(y)) => x.partial_cmp(&(*y as f64)),

            (Float(x), Float(y)) => x.partial_cmp(y),
            (Float(x), Rat(y)) => x.partial_cmp(&f64::from(*y)),
            (Rat(x), Float(y)) => f64::from(*x).partial_cmp(y),

            (Rat(x), Rat(y)) => Some(x.cmp(y)),
        }
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Const::Int(i) => write!(f, "{}", i),
            Const::Float(i) => write!(f, "{:.1}", i),
            Const::Rat(r) => write!(f, "{}", r),
            Const::Inf => write!(f, "inf"),
            Const::NegInf => write!(f, "-inf"),
            Const::Undef => write!(f, "undef"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Func {
    Sin,
    Cos,
    Tan,

    ASin,
    ACos,
    ATan,
    
    Log,
    Sqrt,
    Cbrt,
    Root,

    Func(String),
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Func::Func(s) => write!(f, "{}", s),
            x => write!(f, "{:?}", x),
        }
    }
}

fn print_sep_vec<T: fmt::Display>(v: &Vec<T>, sep: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let iter = v.iter();
    if let Some(ex) = iter.next() {
        write!(f, "{}", *ex)?;
        for ex in iter {
            write!(f, "{}{}", sep, *ex)?;
        }
    }
    Ok(())
}


#[derive(Debug, PartialEq)]
pub enum Expr {
    Sum(Vec<Box<Expr>>),
    Prod(Vec<Box<Expr>>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Recipr(Box<Expr>),
    Call(Func, Vec<Box<Expr>>),
    Const(Const),
    Var(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Sum(v) => { 
                write!(f, "(+ ")?;
                print_sep_vec(v, " ", f)?;
                write!(f, ")")
            },
            Expr::Prod(v) => { 
                write!(f, "(* ")?;
                print_sep_vec(v, " ", f)?;
                write!(f, ")")
            },
            Expr::Pow(lhs, rhs) => write!(f, "(^ {} {})", lhs, rhs),
            Expr::Neg(expr) => write!(f, "(- {})", expr),
            Expr::Recipr(expr) => write!(f, "(/ 1 {})", expr),
            Expr::Call(fun, v) => {
                write!(f, "({} ", fun)?;
                print_sep_vec(v, " ", f)?;
                write!(f, ")")
            },
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Var(s) => write!(f, "{}", s),
        }
    }
}