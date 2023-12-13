#![allow(dead_code)]

use std::ops::Index;

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
    pub fn rows(&self) -> Vec<Vec<&T>> {
        return self
            .data
            .chunks(self.cols)
            .map(|c| c.into_iter().collect())
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
