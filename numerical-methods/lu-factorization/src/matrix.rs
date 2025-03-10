use std::fmt::{Debug, Display};

use crate::Number;

pub struct Matrix<const N: usize>([[Number; N]; N]);

impl<const N: usize> Matrix<N> {
    pub const fn get(&self, (row, col): (usize, usize)) -> Number {
        return self.0[row - 1][col - 1];
    }

    pub const fn set(&mut self, (row, col): (usize, usize), val: Number) {
        self.0[row - 1][col - 1] = val;
    }
}

impl<const N: usize> From<[[Number; N]; N]> for Matrix<N> {
    fn from(value: [[Number; N]; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self([[0.0; N]; N])
    }
}

impl<const N: usize> Display for Matrix<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width = self.0.iter()
            .flatten()
            .map(|num| format!("{:.2}", num).len())
            .max()
            .unwrap_or(1);

        for row in &self.0 {
            for &num in row {
                write!(f, "{:>width$.2}, ", num, width = max_width)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const N: usize> Debug for Matrix<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width = self.0.iter()
            .flatten()
            .map(|num| format!("{:.2}", num).len())
            .max()
            .unwrap_or(1);

        for row in &self.0 {
            for &num in row {
                write!(f, "{:>width$.2}, ", num, width = max_width)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
