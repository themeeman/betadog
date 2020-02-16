use super::expr::{Const, Expr};

fn add_const(lhs: Const, rhs: Const) -> Const {
    match (lhs, rhs) {
        (Const::Int(i), Const::Int(j)) => Const::Int(i + j),
        (Const::Int(i), Const::Float(j)) => Const::Float(i as f64 + j),
        (Const::Float(i), Const::Int(j)) => Const::Float(i + j as f64),
        (Const::Float(i), Const::Float(j)) => Const::Float(i + j),
    }
}

fn sub_const(lhs: Const, rhs: Const) -> Const {
    match (lhs, rhs) {
        (Const::Int(i), Const::Int(j)) => Const::Int(i - j),
        (Const::Int(i), Const::Float(j)) => Const::Float(i as f64 - j),
        (Const::Float(i), Const::Int(j)) => Const::Float(i - j as f64),
        (Const::Float(i), Const::Float(j)) => Const::Float(i - j),
    }
}

fn mul_const(lhs: Const, rhs: Const) -> Const {
    match (lhs, rhs) {
        (Const::Int(i), Const::Int(j)) => Const::Int(i * j),
        (Const::Int(i), Const::Float(j)) => Const::Float(i as f64 * j),
        (Const::Float(i), Const::Int(j)) => Const::Float(i * j as f64),
        (Const::Float(i), Const::Float(j)) => Const::Float(i * j),
    }
}

fn div_const(lhs: Const, rhs: Const) -> Const {
    match (lhs, rhs) {
        (Const::Int(i), Const::Int(j)) => Const::Float(i as f64 / j as f64),
        (Const::Int(i), Const::Float(j)) => Const::Float(i as f64 / j),
        (Const::Float(i), Const::Int(j)) => Const::Float(i / j as f64),
        (Const::Float(i), Const::Float(j)) => Const::Float(i / j),
    }
}

pub fn eval(expr: Expr) -> Const {
    match expr {
        Expr::Add(lhs, rhs) => add_const(eval(*lhs), eval(*rhs)),
        Expr::Sub(lhs, rhs) => sub_const(eval(*lhs), eval(*rhs)),
        Expr::Mul(lhs, rhs) => mul_const(eval(*lhs), eval(*rhs)),
        Expr::Div(lhs, rhs) => div_const(eval(*lhs), eval(*rhs)),
        Expr::Const(c) => c,
    }
}