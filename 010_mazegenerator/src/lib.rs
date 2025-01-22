use graphics_lib::{Drawable, DrawingContext, EventHandler, Runnable, Updatable, UpdateContext};
use opengl_graphics::GlGraphics;
use piston::Size;

mod hexagon_maze;
mod maze;
use hexagon_maze::Maze;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const NUM_ROWS: usize = 50;
const NUM_COLS: usize = 50;

pub struct MazeGenerator {
    maze: Maze<NUM_ROWS, NUM_COLS>,
}

impl MazeGenerator {
    pub fn new() -> MazeGenerator {
        MazeGenerator { maze: Maze::new() }
    }
}

impl Drawable for MazeGenerator {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        self.maze.draw(ctx, gl);
    }
}

impl Updatable for MazeGenerator {
    fn update(&mut self, ctx: &UpdateContext) {
        self.maze.update(ctx);
    }
}

impl EventHandler for MazeGenerator {}

impl Runnable for MazeGenerator {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
