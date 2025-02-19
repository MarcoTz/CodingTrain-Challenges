use graphics::{
    colors::rgba, line, rectangle, Drawable, DrawingContext, EventHandler, Graphics, Runnable,
    Updatable, UpdateContext, WindowConfig,
};
use math::grid::Grid;
use std::cmp::Ordering;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const ROWS: usize = 20;
const COLS: usize = 20;
const MAX_W: usize = 255;

pub type Pos = (usize, usize);
pub type WeightedPos = (Pos, usize);

#[derive(Clone, Copy)]
struct SearchPos {
    pos: Pos,
    weight: usize,
    parent: Pos,
}

impl PartialEq for SearchPos {
    fn eq(&self, other: &SearchPos) -> bool {
        self.pos == other.pos
    }
}

impl Eq for SearchPos {}

impl PartialOrd for SearchPos {
    fn partial_cmp(&self, other: &SearchPos) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Ord for SearchPos {
    fn cmp(&self, other: &SearchPos) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

pub struct AStar {
    weights: Grid<usize>,
    start: Pos,
    end: Pos,
    open: Vec<SearchPos>,
    closed: Vec<SearchPos>,
    done: bool,
}

impl AStar {
    pub fn new() -> AStar {
        let start = (
            rand::random::<usize>() % COLS,
            rand::random::<usize>() % ROWS,
        );
        let end = (
            rand::random::<usize>() % COLS,
            rand::random::<usize>() % ROWS,
        );
        AStar {
            weights: Grid::from_fn(|_, _| rand::random::<usize>() & MAX_W, COLS, ROWS),
            start,
            end,
            open: vec![SearchPos {
                pos: start,
                weight: 0,
                parent: start,
            }],
            closed: Vec::new(),
            done: false,
        }
    }

    fn path_cost(&self, start: Pos, dest: Pos) -> usize {
        let x_min = start.0.min(dest.0);
        let x_max = start.0.max(dest.0);
        let y_min = start.1.min(dest.1);
        let y_max = start.1.max(dest.1);
        let mut total_w = 0;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                total_w += self.weights[(x, y)];
            }
        }
        total_w
    }

    fn heuristic(&self, pos: Pos) -> usize {
        let x_diff = pos.0.abs_diff(self.end.0);
        let y_diff = pos.1.abs_diff(self.end.1);
        x_diff * x_diff + y_diff * y_diff
    }

    fn next_open(&mut self) -> SearchPos {
        let next_ind = self
            .open
            .iter()
            .enumerate()
            .min_by(|(_, pos1), (_, pos2)| pos1.weight.cmp(&pos2.weight))
            .unwrap()
            .0;
        self.open.remove(next_ind)
    }

    fn add_open(&mut self, pos: SearchPos) {
        if self.closed.iter().find(|pos2| **pos2 == pos).is_some() {
            return;
        }

        match self.open.iter_mut().find(|pos2| **pos2 == pos) {
            None => self.open.push(pos),
            Some(op) => {
                if op.weight < pos.weight {
                    return;
                } else {
                    op.weight = pos.weight;
                    op.parent = pos.parent
                }
            }
        }
    }

    fn neighbors(&self, pos: SearchPos) -> Vec<Pos> {
        let (x, y) = (pos.pos.0 as i32, pos.pos.1 as i32);

        let mut pos = vec![];
        let diag_tl = (x > 0 && y > 0).then_some((x - 1, y - 1));
        pos.push(diag_tl);
        let diag_tr = (x < COLS as i32 - 1 && y > 0).then_some((x + 1, y - 1));
        pos.push(diag_tr);
        let top = (y > 0).then_some((x, y - 1));
        pos.push(top);
        let left = (x > 0).then_some((x - 1, y));
        pos.push(left);
        let right = (x < COLS as i32 - 1).then_some((x + 1, y));
        pos.push(right);
        let diag_bl = (x > 0 && y < ROWS as i32 - 1).then_some((x - 1, y + 1));
        pos.push(diag_bl);
        let diag_br = (x < COLS as i32 - 1 && y < ROWS as i32 - 1).then_some((x + 1, y + 1));
        pos.push(diag_br);
        let bottom = (y < ROWS as i32 - 1).then_some((x, y + 1));
        pos.push(bottom);

        pos.into_iter()
            .filter_map(|x| x.map(|(fst, snd)| (fst as usize, snd as usize)))
            .collect()
    }
}

impl Drawable for AStar {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let cell_width = ctx.args.window_size[0] / COLS as f64;
        let cell_height = ctx.args.window_size[1] / ROWS as f64;
        let transform = ctx.id_trans();
        for i in 0..COLS {
            for j in 0..ROWS {
                let x = i as f64 * cell_width;
                let y = j as f64 * cell_height;
                let w = self.weights[(i, j)] as u8;
                let color = rgba::WHITE.with_trans(w);
                rectangle(color.into(), [x, y, cell_width, cell_height], transform, gl);
            }
        }

        if self.done {
            let mut next = self.closed.last();
            while next.is_some() {
                let pos = next.unwrap();
                if pos.pos == self.start {
                    break;
                }
                let x = pos.pos.0 as f64 * cell_width;
                let y = pos.pos.1 as f64 * cell_height;
                rectangle(
                    rgba::GREEN.into(),
                    [x, y, cell_width, cell_height],
                    transform,
                    gl,
                );
                next = self.closed.iter().find(|p| p.pos == pos.parent);
            }
        } else {
            for open in self.open.iter() {
                let o_x = open.pos.0 as f64 * cell_width;
                let o_y = open.pos.1 as f64 * cell_height;
                rectangle(
                    rgba::GREEN.into(),
                    [o_x, o_y, cell_width, cell_height],
                    transform,
                    gl,
                );
            }
            for close in self.closed.iter() {
                let c_x = close.pos.0 as f64 * cell_width;
                let c_y = close.pos.1 as f64 * cell_height;
                rectangle(
                    rgba::RED.into(),
                    [c_x, c_y, cell_width, cell_height],
                    transform,
                    gl,
                );
            }
        }

        let end_x = self.end.0 as f64 * cell_width;
        let end_y = self.end.1 as f64 * cell_height;
        rectangle(
            rgba::BLUE.into(),
            [end_x, end_y, cell_width, cell_height],
            transform,
            gl,
        );
        let start_x = self.start.0 as f64 * cell_width;
        let start_y = self.start.1 as f64 * cell_height;
        rectangle(
            rgba::YELLOW.into(),
            [start_x, start_y, cell_width, cell_height],
            transform,
            gl,
        );
    }
}

impl Updatable for AStar {
    fn update(&mut self, _: &mut UpdateContext) {
        if self.open.is_empty() || self.done {
            return;
        }

        let next = self.next_open();
        if next.pos == self.end {
            self.done = true;
            self.closed.push(next);
            return;
        }
        self.closed.push(next);

        let next_neighbors = self.neighbors(next);
        for next_n in next_neighbors {
            let w = self.heuristic(next_n) + self.path_cost(next.pos, next_n);
            self.add_open(SearchPos {
                pos: next_n,
                weight: w,
                parent: next.pos,
            });
        }
    }
}

impl EventHandler for AStar {}

impl Runnable for AStar {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "AStar".to_owned(),
        }
    }
}
