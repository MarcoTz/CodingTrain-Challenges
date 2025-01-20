use graphics::{clear, rectangle};
use graphics_lib::{Drawable, DrawingContext, InputHandler, Runnable, Updatable, UpdateContext};
use opengl_graphics::GlGraphics;
use piston::ButtonArgs;
use window::Size;

mod food;
mod snake;
use food::Food;
use snake::Snake;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 800.0;
const GRID_SQUARE: f64 = 30.0;

const X_RES: f64 = WIDTH / (GRID_SQUARE);
const Y_RES: f64 = HEIGHT / (GRID_SQUARE);

pub struct SnakeGame {
    food: Food,
    snake: Snake,
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        SnakeGame {
            food: Food::new(),
            snake: Snake::new((X_RES / 2.0).round() as u64, (Y_RES / 2.0).round() as u64),
        }
    }
}

impl Default for SnakeGame {
    fn default() -> SnakeGame {
        SnakeGame::new()
    }
}

impl Updatable for SnakeGame {
    fn update(&mut self, ctx: &UpdateContext) {
        self.snake.update(ctx);
        let snake_pos = self.snake.pos();
        if snake_pos.0 == self.food.x && snake_pos.1 == self.food.y {
            self.food = Food::new();
            self.snake.grow();
        }
        if self.snake.check_death(ctx.window_width, ctx.window_height) {
            std::process::exit(0)
        }
    }
}

impl Drawable for SnakeGame {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.id_trans();
        let bg_light = [0.0, 1.0, 0.0, 1.0];
        let bg_dark = [0.5, 1.0, 0.0, 1.0];
        clear(bg_light, gl);
        let num_cols = (ctx.args.window_size[0] / GRID_SQUARE).ceil() as u64;
        let num_rows = (ctx.args.window_size[1] / GRID_SQUARE).ceil() as u64;
        for i in 0..num_cols {
            for j in 0..num_rows {
                if (i + j) % 2 == 0 {
                    continue;
                }
                let x = i as f64 * GRID_SQUARE;
                let y = j as f64 * GRID_SQUARE;
                rectangle(bg_dark, [x, y, GRID_SQUARE, GRID_SQUARE], transform, gl);
            }
        }

        self.snake.draw(ctx, gl);
        self.food.draw(ctx, gl);
    }
}

impl InputHandler for SnakeGame {
    fn handle(&mut self, args: &ButtonArgs) {
        self.snake.handle(args);
    }
}

impl Runnable for SnakeGame {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
