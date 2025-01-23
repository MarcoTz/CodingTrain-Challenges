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

    pub fn init<F: Fn(usize, usize) -> T>(&mut self, fun: F) {
        for y in 0..self.num_rows {
            for x in 0..self.num_cols {
                self.elements.push(fun(x, y))
            }
        }
    }

    fn ind(&self, x: usize, y: usize) -> usize {
        y * self.num_cols + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.elements.get(self.ind(x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let ind = self.ind(x, y);
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
        let result = grid[1][1];
        let expected = (1, 1);
        assert_eq!(result, expected)
    }

    #[test]
    fn grid_index_tuple() {
        let mut grid = Grid::new(5, 5);
        grid.init(|x, y| (x, y));
        let result = &grid[4];
        let expected = &[(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)];
        assert_eq!(result, expected)
    }
}
