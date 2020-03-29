use std::fmt;
use super::rat::Rat;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Const {
    Int(i128),
    Float(f64),
    Rat(Rat),
    Inf,
    NegInf,
    Undef
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

#[derive(Debug)]
pub enum Expr {
    Sum(Vec<Box<Expr>>),
    Prod(Vec<Box<Expr>>),
    Call(String, Vec<Box<Expr>>),
    Const(Const),
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

fn vec_ptr_eq<T: PartialEq>(lhs: &Vec<Box<T>>, rhs: &Vec<Box<T>>) -> bool {
    (lhs.len() == rhs.len()) &&
    lhs.iter()
        .zip(rhs)
        .all(|(a, b)| *a == *b)
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
            Expr::Call(s, v) => {
                write!(f, "({} ", s)?;
                print_sep_vec(v, " ", f)?;
                write!(f, ")")
            }
            Expr::Const(c) => write!(f, "{}", c)
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (Sum(lhs), Sum(rhs)) => vec_ptr_eq(lhs, rhs),
            (Prod(lhs), Prod(rhs)) => vec_ptr_eq(lhs, rhs),
            (Call(s1, lhs), Call(s2, rhs)) => s1 == s2 && vec_ptr_eq(lhs, rhs),
            (Const(lhs), Const(rhs)) => lhs == rhs,
            (_, _) => false
        }
    }
}