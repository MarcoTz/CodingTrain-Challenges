use core::array;
use graphics::{line, rectangle};
use graphics_lib::{Drawable, DrawingContext, Graphics, Updatable, UpdateContext};

pub struct Walls {
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
}

pub struct MazeCell {
    walls: Walls,
    visited: bool,
}

impl MazeCell {
    pub fn new() -> MazeCell {
        MazeCell {
            walls: Walls {
                top: true,
                right: true,
                bottom: true,
                left: true,
            },
            visited: false,
        }
    }
}

pub struct Maze<const ROWS: usize, const COLS: usize> {
    cells: [[MazeCell; ROWS]; COLS],
    next_unvisited: (usize, usize),
    last_visited: Vec<(usize, usize)>,
    done: bool,
}

impl<const ROWS: usize, const COLS: usize> Maze<ROWS, COLS> {
    pub fn new() -> Maze<ROWS, COLS> {
        Maze {
            cells: array::from_fn(|_| array::from_fn(|_| MazeCell::new())),
            next_unvisited: (0, 0),
            last_visited: vec![],
            done: false,
        }
    }

    fn unvisited_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if x + 1 < COLS && !self.cells[x + 1][y].visited {
            neighbors.push((x + 1, y));
        }
        if x > 0 && !self.cells[x - 1][y].visited {
            neighbors.push((x - 1, y));
        }

        if y + 1 < ROWS && !self.cells[x][y + 1].visited {
            neighbors.push((x, y + 1));
        }

        if y > 0 && !self.cells[x][y - 1].visited {
            neighbors.push((x, y - 1));
        }
        neighbors
    }

    fn update_walls(&mut self, x: usize, y: usize, new_x: usize, new_y: usize) {
        if new_x > x {
            self.cells[x][y].walls.right = false;
            self.cells[new_x][new_y].walls.left = false;
        } else if new_x < x {
            self.cells[x][y].walls.left = false;
            self.cells[new_x][new_y].walls.right = false;
        } else if new_y > y {
            self.cells[x][y].walls.bottom = false;
            self.cells[new_x][new_y].walls.top = false;
        } else if new_y < y {
            self.cells[x][y].walls.top = false;
            self.cells[new_x][new_y].walls.bottom = false;
        }
    }

    fn backtrack(&mut self) {
        let mut prev = self.next_unvisited;
        let mut prev_neighbors = vec![];
        while prev_neighbors.is_empty() {
            prev = self.last_visited.pop().unwrap();
            prev_neighbors = self.unvisited_neighbors(prev.0, prev.1);
        }
        self.next_unvisited = prev;
    }
}

impl<const ROWS: usize, const COLS: usize> Drawable for Maze<ROWS, COLS> {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let cell_height = (ctx.args.window_size[1] / (ROWS as f64)).floor();
        let cell_width = (ctx.args.window_size[0] / (COLS as f64)).floor();

        let transform = ctx.id_trans();
        let color = [1.0, 1.0, 1.0, 1.0];
        let thickness = 1.0;
        for x in 0..ROWS {
            for y in 0..COLS {
                let top_left_x = x as f64 * cell_width;
                let top_left_y = y as f64 * cell_height;
                let cell = &self.cells[x][y];

                if cell.visited && !self.done {
                    rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        [top_left_x, top_left_y, cell_width, cell_height],
                        transform,
                        gl,
                    );
                }

                if (x, y) == self.next_unvisited && !self.done {
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0],
                        [top_left_x, top_left_y, cell_width, cell_height],
                        transform,
                        gl,
                    );
                }

                if cell.walls.top {
                    line(
                        color,
                        thickness,
                        [top_left_x, top_left_y, top_left_x + cell_width, top_left_y],
                        transform,
                        gl,
                    );
                }

                if cell.walls.bottom {
                    line(
                        color,
                        thickness,
                        [
                            top_left_x,
                            top_left_y + cell_height,
                            top_left_x + cell_width,
                            top_left_y + cell_height,
                        ],
                        transform,
                        gl,
                    );
                }

                if cell.walls.left {
                    line(
                        color,
                        thickness,
                        [top_left_x, top_left_y, top_left_x, top_left_y + cell_height],
                        transform,
                        gl,
                    );
                }

                if cell.walls.right {
                    line(
                        color,
                        thickness,
                        [
                            top_left_x + cell_width,
                            top_left_y,
                            top_left_x + cell_width,
                            top_left_y + cell_height,
                        ],
                        transform,
                        gl,
                    );
                }
            }
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Updatable for Maze<ROWS, COLS> {
    fn update(&mut self, _: &UpdateContext) {
        if self.done {
            return;
        }

        let (x, y) = self.next_unvisited;
        self.cells[x][y].visited = true;

        if self
            .cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.visited))
        {
            self.done = true;
            return;
        }

        let neighbors = self.unvisited_neighbors(x, y);

        if neighbors.len() == 0 {
            self.backtrack();
            return;
        }

        self.last_visited.push((x, y));
        let next_index = rand::random::<usize>() % neighbors.len();
        let (new_x, new_y) = neighbors[next_index];
        self.next_unvisited = neighbors[next_index];
        self.update_walls(x, y, new_x, new_y);
    }
}
