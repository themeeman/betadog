use super::expr::{Const, Expr};

fn add_const(lhs: Const, rhs: Const) -> Const {
    use Const::*;
    match (lhs, rhs) {
        (Int(i), Int(j)) => Int(i + j),
        (Int(i), Float(j)) => Float(i as f64 + j),
        (Float(i), Int(j)) => Float(i + j as f64),
        (Float(i), Float(j)) => Float(i + j),
    }
}

fn sub_const(lhs: Const, rhs: Const) -> Const {
    use Const::*;
    match (lhs, rhs) {
        (Int(i), Int(j)) => Int(i - j),
        (Int(i), Float(j)) => Float(i as f64 - j),
        (Float(i), Int(j)) => Float(i - j as f64),
        (Float(i), Float(j)) => Float(i - j),
    }
}

fn mul_const(lhs: Const, rhs: Const) -> Const {
    use Const::*;
    match (lhs, rhs) {
        (Int(i), Int(j)) => Int(i * j),
        (Int(i), Float(j)) => Float(i as f64 * j),
        (Float(i), Int(j)) => Float(i * j as f64),
        (Float(i), Float(j)) => Float(i * j),
    }
}

fn div_const(lhs: Const, rhs: Const) -> Const {
    use Const::*;
    match (lhs, rhs) {
        (Int(i), Int(j)) => Float(i as f64 / j as f64),
        (Int(i), Float(j)) => Float(i as f64 / j),
        (Float(i), Int(j)) => Float(i / j as f64),
        (Float(i), Float(j)) => Float(i / j),
    }
}

fn negate_const(expr: Const) -> Const {
    use Const::*;
    match expr {
        Int(i) => Int(-i),
        Float(i) => Float(-i),
    }
}

pub fn eval(expr: Expr) -> Const {
    match expr {
        Expr::Add(lhs, rhs) => add_const(eval(*lhs), eval(*rhs)),
        Expr::Sub(lhs, rhs) => sub_const(eval(*lhs), eval(*rhs)),
        Expr::Mul(lhs, rhs) => mul_const(eval(*lhs), eval(*rhs)),
        Expr::Div(lhs, rhs) => div_const(eval(*lhs), eval(*rhs)),
        Expr::Neg(expr) => negate_const(eval(*expr)),
        Expr::Null(expr) => eval(*expr),
        Expr::Const(c) => c,
    }
}