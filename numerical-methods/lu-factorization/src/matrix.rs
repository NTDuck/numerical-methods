use std::{fmt::{Debug, Display}, ops::{Deref, Index}, slice::{Iter, IterMut}};

use crate::{Number, RowVector};

pub struct Matrix<const N: usize>(Box<[RowVector<N>; N]>);

impl<const N: usize> Matrix<N> {
    pub fn iter(&self) -> Iter<RowVector<N>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<RowVector<N>> {
        self.0.iter_mut()
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self(Box::new([RowVector::from(Default::default()); N]))
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
