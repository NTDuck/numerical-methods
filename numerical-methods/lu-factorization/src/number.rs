use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{self, Display, Debug};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Number {
    pub const fn new(val: f64) -> Self {
        return Self(val);
    }

    const ZERO: Self = Self::new(0.0);
    const ONE: Self = Self::new(1.0);
    const INF: Self = Self::new(f64::INFINITY);
    const NEG_INF: Self = Self::new(f64::NEG_INFINITY);
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Number(self.0 + other.0)
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Number(self.0 - other.0)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Number(self.0 * other.0)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Number(self.0 / other.0)
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Number(-self.0)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}
