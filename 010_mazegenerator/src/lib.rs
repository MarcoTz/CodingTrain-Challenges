use graphics_lib::{
    Drawable, DrawingContext, EventHandler, InputContext, Runnable, Updatable, UpdateContext,
};
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonState, Key, Size};

mod hexagon_maze;
mod maze;
use hexagon_maze::Maze as HexagonMaze;
use maze::Maze as SquareMaze;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const NUM_ROWS: usize = 50;
const NUM_COLS: usize = 50;

pub struct MazeGenerator {
    hexagon_maze: HexagonMaze<NUM_ROWS, NUM_COLS>,
    square_maze: SquareMaze<NUM_ROWS, NUM_COLS>,
    show_hexagon: bool,
}

impl MazeGenerator {
    pub fn new() -> MazeGenerator {
        MazeGenerator {
            hexagon_maze: HexagonMaze::new(),
            square_maze: SquareMaze::new(),
            show_hexagon: true,
        }
    }
}

impl Drawable for MazeGenerator {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        if self.show_hexagon {
            self.hexagon_maze.draw(ctx, gl);
        } else {
            self.square_maze.draw(ctx, gl);
        }
    }
}

impl Updatable for MazeGenerator {
    fn update(&mut self, ctx: &UpdateContext) {
        self.hexagon_maze.update(ctx);
        self.square_maze.update(ctx);
    }
}

impl EventHandler for MazeGenerator {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.button == Button::Keyboard(Key::Space) && ctx.args.state == ButtonState::Release
        {
            self.show_hexagon = !self.show_hexagon;
        }
    }
}

impl Runnable for MazeGenerator {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
