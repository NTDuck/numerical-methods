use std::fmt::{Debug, Display};

use crate::Number;

pub struct ColumnVector<const N: usize>([Number; N]);

impl<const N: usize> ColumnVector<N> {
    pub const fn get(&self, row: usize) -> Number {
        return self.0[row - 1];
    }

    pub const fn set(&mut self, row: usize, val: Number) {
        self.0[row - 1] = val;
    }
}


impl<const N: usize> From<[Number; N]> for ColumnVector<N> {
    fn from(value: [Number; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Default for ColumnVector<N> {
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> Display for ColumnVector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width = self.0.iter()
            .map(|num| format!("{:.2}", num).len())
            .max()
            .unwrap_or(1);

        for row in &self.0 {
            writeln!(f, "{:>width$.2}, ", row, width = max_width)?;
        }

        Ok(())
    }
}

impl<const N: usize> Debug for ColumnVector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width = self.0.iter()
            .map(|num| format!("{:.2}", num).len())
            .max()
            .unwrap_or(1);

        for row in &self.0 {
            writeln!(f, "{:>width$.2}, ", row, width = max_width)?;
        }

        Ok(())
    }
}
