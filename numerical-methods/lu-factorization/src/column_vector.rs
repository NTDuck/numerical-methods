use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::Vector;

pub struct ColumnVector<const N: usize>(Vector<N>);

impl<const N: usize> From<Vector<N>> for ColumnVector<N> {
    fn from(vector: Vector<N>) -> Self {
        Self(vector)
    }
}

impl<const N: usize> Deref for ColumnVector<N> {
    type Target = Vector<N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ColumnVector<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Display for ColumnVector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width = self.0.iter()
            .map(|num| format!("{:.2}", num).len())
            .max()
            .unwrap_or(1);

        for num in &self.0 {
            writeln!(f, "{:>width$.2}, ", num, width = max_width)?;
        }

        Ok(())
    }
}
