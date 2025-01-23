use graphics::types::Color;
use graphics_lib::{
    grid::Grid, Drawable, DrawingContext, EventHandler, InputContext, Runnable, SetupContext,
    Updatable, UpdateContext,
};
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonState, Key, MouseButton, ResizeArgs, Size};
//use std::time::Instant;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const NUM_ROWS: usize = 300;
const NUM_COLS: usize = 300;

const FEED_RATE: f64 = 0.055;
const KILL_RATE: f64 = 0.062;
const DIFFUSION_A: f64 = 1.0;
const DIFFUSION_B: f64 = 0.5;
const COLOR_B: Color = [1.0, 1.0, 1.0, 1.0];
const COLOR_A: Color = [0.0, 0.0, 0.0, 1.0];

const LAPLACE_WEIGHTS: [[f64; 3]; 3] = [[0.05, 0.2, 0.05], [0.2, -1.0, 0.2], [0.05, 0.2, 0.05]];

const MOUSE_BRUSH_SIZE: usize = 10;
const NUM_UPDATES: usize = 10;

const HELP_TEXT: &str = "Controls:
h: show help text (stdout)
spacebar: pause/unpause,
r ; reset
";

mod cell;
use cell::Cell;

pub fn interpolate_color(concentration_a: f32, concentration_b: f32) -> Color {
    [
        COLOR_A[0] * concentration_a + COLOR_B[0] * concentration_b,
        COLOR_A[1] * concentration_a + COLOR_B[1] * concentration_b,
        COLOR_A[2] * concentration_a + COLOR_B[2] * concentration_b,
        1.0,
    ]
}

pub struct ReactionDiffusion {
    running: bool,
    drawing: bool,
    cells: Grid<Cell>,
}

impl ReactionDiffusion {
    pub fn new() -> ReactionDiffusion {
        ReactionDiffusion {
            drawing: false,
            running: false,
            cells: Grid::from_fn(|x, y| Cell::new(x as u64, y as u64), NUM_COLS, NUM_ROWS),
        }
    }

    fn resize_cells(&mut self, window_width: f64, window_height: f64) {
        for cell in self.cells.iter_mut() {
            let width = window_width / NUM_COLS as f64;
            let height = window_height / NUM_ROWS as f64;
            cell.width = width;
            cell.height = height;
        }
    }

    pub fn mouse_draw(&mut self, window_width: f64, window_height: f64, mouse_pos: [f64; 2]) {
        let cell_width = window_width / NUM_COLS as f64;
        let cell_height = window_height / NUM_ROWS as f64;
        let x = (mouse_pos[0] / cell_width).round() as usize;
        let y = (mouse_pos[1] / cell_height).round() as usize;
        let min_x = x.saturating_sub(MOUSE_BRUSH_SIZE / 2);
        let max_x = (x + MOUSE_BRUSH_SIZE / 2).min(NUM_COLS - 1);
        let min_y = y.saturating_sub(MOUSE_BRUSH_SIZE / 2);
        let max_y = (y + MOUSE_BRUSH_SIZE / 2).min(NUM_ROWS - 1);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.cells[(x, y)].concentration_a = 0.0;
                self.cells[(x, y)].concentration_b = 1.0;
            }
        }
    }
}

impl Drawable for ReactionDiffusion {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        for cell in self.cells.iter() {
            cell.draw(ctx, gl);
        }
    }
}

impl Updatable for ReactionDiffusion {
    fn update(&mut self, ctx: &UpdateContext) {
        if self.drawing {
            self.mouse_draw(ctx.window_width, ctx.window_height, ctx.mouse_pos);
        }
        if !self.running {
            return;
        }

        for _ in 0..NUM_UPDATES {
            let laplace_a = self
                .cells
                .clone()
                .map(|cell| cell.concentration_a)
                .convolute(LAPLACE_WEIGHTS);
            let laplace_b = self
                .cells
                .clone()
                .map(|cell| cell.concentration_b)
                .convolute(LAPLACE_WEIGHTS);

            for y in 0..NUM_ROWS {
                for x in 0..NUM_COLS {
                    self.cells[(x, y)].laplace_a = laplace_a[(x, y)];
                    self.cells[(x, y)].laplace_b = laplace_b[(x, y)];
                    self.cells[(x, y)].update(ctx);
                }
            }
        }
    }
}

impl EventHandler for ReactionDiffusion {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.button == Button::Mouse(MouseButton::Left) {
            self.drawing = ctx.args.state == ButtonState::Press;
            return;
        }

        if ctx.args.state != ButtonState::Release {
            return;
        }

        if let Button::Keyboard(key) = ctx.args.button {
            match key {
                Key::Space => self.running = !self.running,
                Key::H => println!("{}", HELP_TEXT),
                Key::R => {
                    self.drawing = false;
                    self.running = false;
                    for cell in self.cells.iter_mut() {
                        cell.concentration_a = 1.0;
                        cell.concentration_b = 0.0;
                    }
                }
                _ => return,
            }
        }
    }
    fn handle_resize(&mut self, args: &ResizeArgs) {
        self.resize_cells(args.window_size[0], args.window_size[1]);
    }
}

impl Runnable for ReactionDiffusion {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }

    fn setup(&mut self, ctx: &SetupContext) {
        self.resize_cells(ctx.window_width, ctx.window_height);
    }
}
