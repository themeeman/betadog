use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::cmp::{Ordering};

fn gcd(mut lhs: i128, mut rhs: i128) -> i128 {
    loop {
        match (lhs, rhs) {
            (lhs, 0) => { return lhs; },
            _ => {
                let temp = rhs;
                rhs = lhs % rhs;
                lhs = temp;
            },
        };
    }
}

fn simplify(frac: (i128, i128)) -> (i128, i128) {
    let gcd = gcd(frac.0, frac.1);
    let num = frac.0 / gcd;
    let den = frac.1 / gcd;

    if den < 0 && num > 0 {
        (-num, -den)
    } else {
        (num, den)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rat {
    num: i128,
    den: i128,
}

impl Rat {
    pub fn new(num: i128, den: i128) -> Self {
        if den == 0 {
            panic!("denominator == 0");
        }
        let (num, den) = simplify((num, den));
        Rat{num: num, den: den}
    }

    pub fn num(self) -> i128 {
        self.num
    }

    pub fn den(self) -> i128 {
        self.den
    }
}

impl From<i128> for Rat {
    fn from(x: i128) -> Self {
        Rat{num: x, den: 1}
    }
}

impl From<Rat> for f64 {
    fn from(x: Rat) -> f64 {
        x.num as f64 / x.den as f64
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rat {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.num * other.den;
        let rhs = other.num * self.den;
        if lhs < rhs {
            Ordering::Less
        } else if lhs > rhs {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Add for Rat {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl AddAssign for Rat {
    fn add_assign(&mut self, other: Self) {
        let frac = simplify((self.num * other.den + other.num * self.den, self.den * other.den));
        self.num = frac.0;
        self.den = frac.1;
    }
}

impl Sub for Rat {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl SubAssign for Rat {
    fn sub_assign(&mut self, other: Self) {
        let frac = simplify((self.num * other.den - other.num * self.den, self.den * other.den));
        self.num = frac.0;
        self.den = frac.1;
    }
}

impl Mul for Rat {
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        self *= other;
        self
    }
}

impl MulAssign for Rat {
    fn mul_assign(&mut self, other: Self) {
        let frac = simplify((self.num * other.num, self.den * other.den));
        self.num = frac.0;
        self.den = frac.1;
    }
}

impl Div for Rat {
    type Output = Self;
    fn div(mut self, other: Self) -> Self {
        self /= other;
        self
    }
}

impl DivAssign for Rat {
    fn div_assign(&mut self, other: Self) {
        if other.num() == 0 {
            panic!("Attempt to divide by 0");
        }
        let frac = simplify((self.num * other.den, self.den * other.num));
        self.num = frac.0;
        self.den = frac.1;
    }
}

impl Neg for Rat {
    type Output = Self;
    fn neg(self) -> Rat {
        Rat{num: -self.num, den: self.den}
    }
}