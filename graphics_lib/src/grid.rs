use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    num_rows: usize,
    num_cols: usize,
    elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(num_cols: usize, num_rows: usize) -> Grid<T> {
        Grid {
            num_rows,
            num_cols,
            elements: Vec::with_capacity(num_rows * num_cols),
        }
    }

    pub fn from_default(num_cols: usize, num_rows: usize) -> Grid<T>
    where
        T: Default,
    {
        let mut grid = Grid::new(num_cols, num_rows);
        grid.init(|_, _| T::default());
        grid
    }

    pub fn init<F: Fn(usize, usize) -> T>(&mut self, fun: F) {
        for y in 0..self.num_rows {
            for x in 0..self.num_cols {
                self.elements.push(fun(x, y))
            }
        }
    }

    fn ind(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.num_cols || y >= self.num_rows {
            return None;
        }
        Some(y * self.num_cols + x)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.elements.get(self.ind(x, y)?)
    }

    pub fn get_or_default(&self, x: i64, y: i64) -> T
    where
        T: Default + Clone,
    {
        if x < 0 || y < 0 {
            return T::default();
        }
        self.get(x as usize, y as usize)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let ind = self.ind(x, y)?;
        self.elements.get_mut(ind)
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.elements.iter_mut()
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.elements.iter()
    }

    pub fn row(&self, y: usize) -> &[T] {
        let start_ind = self.num_cols * y;
        &self.elements[start_ind..start_ind + self.num_cols]
    }
}

impl Grid<f64> {
    pub fn convolute<const N: usize, const M: usize>(&self, kernel: [[f64; N]; M]) -> Grid<f64> {
        assert!(N % 2 == 1);
        assert!(M % 2 == 1);
        let mut new_grid = Grid::from_default(self.num_cols, self.num_rows);
        for center_y in 0..self.num_rows {
            for center_x in 0..self.num_cols {
                let start_x = center_x as i64 - (N as i64 - 1) / 2;
                let start_y = center_y as i64 - (M as i64 - 1) / 2;
                for kernel_y in 0..M {
                    let y = start_y + kernel_y as i64;
                    for kernel_x in 0..N {
                        let x = start_x + kernel_x as i64;
                        new_grid[(center_x, center_y)] +=
                            kernel[kernel_x][kernel_y] * self.get_or_default(x, y);
                    }
                }
            }
        }
        new_grid
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.num_rows {
            for x in 0..self.num_cols {
                write!(f, "{} ", self.get(x, y).unwrap())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(x, y).unwrap()
    }
}

impl<'a, T: 'a> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[cfg(test)]
mod grid_tests {
    use super::Grid;
    use crate::vec2d::Vec2D;

    #[test]
    fn init_grid() {
        let mut result = Grid::new(2, 2);
        result.init(|x, y| (x, y));
        let expected = Grid {
            num_rows: 2,
            num_cols: 2,
            elements: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn get_grid() {
        let mut grid = Grid::new(5, 5);
        grid.init(|x, y| (x, y));
        let result = grid.get(3, 4).unwrap();
        let expected = &(3, 4);
        assert_eq!(result, expected);
    }

    #[test]
    fn display_grid() {
        let mut grid = Grid::new(2, 2);
        grid.init(|x, y| Vec2D::new(x as f64, y as f64));
        let result = format!("{grid}");
        let expected = "(0,0) (1,0) \n(0,1) (1,1) \n";
        assert_eq!(result, expected)
    }

    #[test]
    fn get_row() {
        let mut grid = Grid::new(5, 5);
        grid.init(|x, y| (x, y));
        let result = grid.row(3);
        let expected = &[(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)];
        assert_eq!(result, expected)
    }

    #[test]
    fn grid_index() {
        let mut grid = Grid::new(5, 5);
        grid.init(|x, y| (x, y));
        let result = grid[(1, 1)];
        let expected = (1, 1);
        assert_eq!(result, expected)
    }

    #[test]
    fn convolution() {
        let mut grid = Grid::new(3, 3);
        grid.init(|_, _| 1.0);
        let result = grid.convolute([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
        let expected = Grid {
            num_rows: 3,
            num_cols: 3,
            elements: vec![4.0, 6.0, 4.0, 6.0, 9.0, 6.0, 4.0, 6.0, 4.0],
        };
        assert_eq!(result, expected)
    }
}
