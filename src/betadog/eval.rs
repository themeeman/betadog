use super::expr::{Const, Expr};
use super::rat;

fn eval_rat(r: rat::Rat) -> Const {
    match r.den() {
        1 => Const::Int(r.num()),
        -1 => Const::Int(-r.num()),
        _ => Const::Rat(r),
    }
}

fn add_const(lhs: Const, rhs: Const) -> Const {
    use Const::*;
    match (lhs, rhs) {
        (Undef, _) => Undef,
        (_, Undef) => Undef,

        (Int(i), Int(j)) => Int(i + j),
        (Int(i), Float(j)) => Float(i as f64 + j),
        (Float(i), Int(j)) => Float(i + j as f64),
        (Float(i), Float(j)) => Float(i + j),
        (Rat(i), Int(j)) => eval_rat(i + rat::Rat::from(j)),
        (Rat(i), Float(j)) => Float(f64::from(i) + j),
        (Int(i), Rat(j)) => eval_rat(rat::Rat::from(i) + j),
        (Float(i), Rat(j)) => Float(i + f64::from(j)),
        (Rat(i), Rat(j)) => eval_rat(i + j),
        
        (Inf, NegInf) => Undef,
        (NegInf, Inf) => Undef,
        (_, Inf) => Inf,
        (Inf, _) => Inf,
        (_, NegInf) => NegInf,
        (NegInf, _) => NegInf,

    }
}

fn sub_const(lhs: Const, rhs: Const) -> Const {
    add_const(lhs, negate_const(rhs))
}

fn mul_const(lhs: Const, rhs: Const) -> Const {
    use Const::*;
    match (lhs, rhs) {
        (Undef, _) => Undef,
        (_, Undef) => Undef,

        (Int(i), Int(j)) => Int(i * j),
        (Int(i), Float(j)) => Float(i as f64 * j),
        (Float(i), Int(j)) => Float(i * j as f64),
        (Float(i), Float(j)) => Float(i * j),
        (Rat(i), Int(j)) => eval_rat(i * rat::Rat::from(j)),
        (Rat(i), Float(j)) => Float(f64::from(i) * j),
        (Int(i), Rat(j)) => eval_rat(rat::Rat::from(i) * j),
        (Float(i), Rat(j)) => Float(i * f64::from(j)),
        (Rat(i), Rat(j)) => eval_rat(i * j),

        (Inf, x) => mul_infinity_const(x),
        (x, Inf) => mul_infinity_const(x),

        (NegInf, x) => mul_infinity_const(negate_const(x)),
        (x, NegInf) => mul_infinity_const(negate_const(x)),
    }
}

fn div_const(lhs: Const, rhs: Const) -> Const {
    mul_const(lhs, reciprocal_const(rhs))
}

fn negate_const(expr: Const) -> Const {
    use Const::*;
    match expr {
        Int(i) => Int(-i),
        Float(i) => Float(-i),
        Rat(r) => Rat(-r),
        Inf => NegInf,
        NegInf => Inf,
        Undef => Undef,
    }
}

fn reciprocal_const(expr: Const) -> Const {
    use Const::*;
    match expr {
        Int(0) => Inf,
        Int(i) => Rat(rat::Rat::new(1, i)),
        Float(i) if i == 0.0 => Inf,
        Float(i) if i == -0.0 => NegInf,
        Float(i) =>  Float(1.0 / i),
        Rat(i) => Rat(rat::Rat::from(1) / i),
        Inf => Int(0),
        NegInf => Int(0),
        Undef => Undef
    }
}

fn mul_infinity_const(expr: Const) -> Const {
    use Const::*;
    const INT_MAX: i128 = std::i128::MAX;
    const INT_MIN: i128 = std::i128::MIN;

    match expr {
        Int(INT_MIN..=-1) => NegInf,
        Int(0) => Undef,
        Int(1..=INT_MAX) => Inf,

        Float(x) if x < 0.0 => NegInf,
        Float(x) if x > 0.0 => Inf,
        Float(_) => Undef,

        Rat(x) if x.num() < 0 => NegInf,
        Rat(x) if x.num() > 0 => Inf,
        Rat(_) => Undef,

        Inf => Inf,
        NegInf => NegInf,
        Undef => Undef,
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