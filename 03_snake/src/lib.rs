use graphics_lib::{Drawable, InputHandler, Runnable, Updatable};
use window::Size;

mod food;
mod snake;
use food::Food;
use snake::Snake;

const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 600.0;
const GRID_SQUARE: f64 = 10.0;

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
            snake: Snake::new(),
        }
    }
}

impl Runnable for SnakeGame {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }

    fn to_draw(&self) -> Vec<&dyn Drawable> {
        vec![&self.food, &self.snake]
    }

    fn to_update(&mut self) -> Vec<&mut dyn Updatable> {
        vec![&mut self.snake]
    }

    fn handlers(&mut self) -> Vec<&mut dyn InputHandler> {
        vec![&mut self.snake]
    }
}
