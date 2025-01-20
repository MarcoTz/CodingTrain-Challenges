use super::GRID_SQUARE;
use graphics::{rectangle, types::Color};
use graphics_lib::{Drawable, DrawingContext, InputHandler, Updatable, UpdateContext};
use opengl_graphics::GlGraphics;

pub struct Snake {
    x: f64,
    y: f64,
    color: Color,
    dir: Dir,
    speed: f64,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            x: 0.0,
            y: 0.0,
            dir: Dir::Right,
            color: [1.0, 0.0, 1.0, 1.0],
            speed: 0.5,
        }
    }
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Drawable for Snake {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.center_trans();
        let x = self.x.round() * GRID_SQUARE;
        let y = self.y.round() * GRID_SQUARE;
        rectangle(self.color, [x, y, GRID_SQUARE, GRID_SQUARE], transform, gl);
    }
}

impl Updatable for Snake {
    fn update(&mut self, _: &UpdateContext) {
        match self.dir {
            Dir::Left => self.x -= self.speed,
            Dir::Right => self.x += self.speed,
            Dir::Up => self.y -= self.speed,
            Dir::Down => self.y += self.speed,
        }
    }
}

impl InputHandler for Snake {}
