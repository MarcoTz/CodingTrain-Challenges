use super::{GRID_SQUARE, X_RES, Y_RES};
use graphics::{rectangle, types::Color};
use graphics_lib::{Drawable, DrawingContext};
use opengl_graphics::GlGraphics;

pub struct Food {
    x: i32,
    y: i32,
    color: Color,
}

impl Food {
    pub fn new() -> Food {
        let x = (X_RES * 0.5 * (rand::random::<f64>() - 1.0)).round() as i32;
        let y = (Y_RES * 0.5 * (rand::random::<f64>() - 1.0)).round() as i32;
        Food {
            x,
            y,
            color: [0.0, 1.0, 1.0, 1.0],
        }
    }
}

impl Drawable for Food {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.center_trans();
        let x = self.x as f64 * GRID_SQUARE;
        let y = self.y as f64 * GRID_SQUARE;
        rectangle(self.color, [x, y, GRID_SQUARE, GRID_SQUARE], transform, gl);
    }
}
