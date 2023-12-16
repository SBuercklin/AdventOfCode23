#![allow(dead_code)]

use core::fmt;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use itertools::Itertools;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AoCMatrix<T>
where
    T: std::clone::Clone,
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> AoCMatrix<T>
where
    T: std::clone::Clone,
{
    pub fn from_rows(mut row_data: Vec<Vec<T>>) -> AoCMatrix<T> {
        let rows = row_data.len();
        let cols = row_data[0].len();

        let data = row_data
            .iter_mut()
            .reduce(|a, b| {
                a.append(b);
                a
            })
            .unwrap()
            .to_vec();

        return AoCMatrix { data, rows, cols };
    }
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn n_rows(&self) -> usize {
        self.rows
    }
    pub fn n_cols(&self) -> usize {
        self.cols
    }
    pub fn in_mat(&self, pos: (usize, usize)) -> bool {
        return pos.0 < self.rows && pos.1 < self.cols;
    }

    pub fn rows(&self) -> Vec<Vec<&T>> {
        return self
            .data
            .chunks(self.cols)
            .map(|c| c.into_iter().collect())
            .collect();
    }
    pub fn rows_mut(&mut self) -> Vec<Vec<&mut T>> {
        return self
            .data
            .chunks_mut(self.cols)
            .map(|c| c.iter_mut().collect())
            .collect();
    }

    pub fn cols(&self) -> Vec<Vec<&T>> {
        let mut cols: Vec<Vec<&T>> = vec![];
        for c in 0..self.cols {
            let mut v = vec![];
            for r in 0..self.rows {
                v.push(&self[(r, c)]);
            }
            cols.push(v);
        }
        return cols;
    }
    pub fn cols_mut(&mut self) -> Vec<Vec<&mut T>> {
        let mut cols: Vec<Vec<&mut T>> = vec![];
        for _ in 0..self.cols {
            cols.push(vec![]);
        }

        let mut data = self.data.iter_mut();

        for _ in 0..self.rows {
            for c in 0..self.cols {
                cols[c].push(data.next().unwrap());
            }
        }

        return cols;
    }
}

impl<T: Copy> AoCMatrix<T> {
    pub fn rows_by_value(&self) -> Vec<Vec<T>> {
        return self.data.chunks(self.cols).map(|c| c.into()).collect();
    }

    pub fn cols_by_value(&self) -> Vec<Vec<T>> {
        let mut cols: Vec<Vec<T>> = vec![];
        for c in 0..self.cols {
            let mut v = vec![];
            for r in 0..self.rows {
                v.push(self[(r, c)]);
            }
            cols.push(v);
        }
        return cols;
    }
}

impl<T> Index<(usize, usize)> for AoCMatrix<T>
where
    T: std::clone::Clone,
{
    type Output = T;

    fn index(&self, pair: (usize, usize)) -> &Self::Output {
        let (row, col) = pair;
        let cols = self.cols;
        let idx = row * cols + col;

        return &self.data[idx];
    }
}

impl<T> IndexMut<(usize, usize)> for AoCMatrix<T>
where
    T: std::clone::Clone,
{
    fn index_mut(&mut self, pair: (usize, usize)) -> &mut Self::Output {
        let (row, col) = pair;
        let cols = self.cols;
        let idx = row * cols + col;

        return &mut self.data[idx];
    }
}

impl<T: Clone + Display> Display for AoCMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rows()
                .iter()
                .map(|r| r.iter().map(|c| c.to_string()).join(""))
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AoCMatrix;

    #[test]
    fn round_trip() {
        let data: Vec<Vec<u64>> = (0..10)
            .collect::<Vec<u64>>()
            .chunks(2)
            .into_iter()
            .map(|c| c.to_vec())
            .collect();

        let mat = AoCMatrix::from_rows(data);

        assert_eq!(mat.rows, 5);
        assert_eq!(mat.cols, 2);

        assert_eq!(mat[(0, 0)], 0);
        assert_eq!(mat[(2, 1)], 5);
    }
}
