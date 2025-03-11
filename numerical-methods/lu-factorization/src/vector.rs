use std::{ops::{Index, IndexMut}, slice::{Iter, IterMut}};

use crate::Number;

#[derive(Debug, Clone)]
pub struct Vector<const N: usize>(Box<[Number; N]>);

impl<const N: usize> Vector<N> {
    pub fn iter(&self) -> Iter<Number> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Number> {
        self.0.iter_mut()
    }
}

impl<const N: usize> From<Box<[Number; N]>> for Vector<N> {
    fn from(vector: Box<[Number; N]>) -> Self {
        Self(vector)
    }
}

impl<const N: usize> From<[Number; N]> for Vector<N> {
    fn from(vector: [Number; N]) -> Self {
        Self(Box::new(vector))
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self(Box::new([Default::default(); N]))
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = Number;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> IntoIterator for Vector<N> {
    type Item = Number;
    type IntoIter = std::array::IntoIter<Number, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Vector<N> {
    type Item = &'a Number;
    type IntoIter = Iter<'a, Number>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut Vector<N> {
    type Item = &'a mut Number;
    type IntoIter = IterMut<'a, Number>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
