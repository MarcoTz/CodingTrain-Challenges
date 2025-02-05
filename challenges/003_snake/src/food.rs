use super::{GRID_SQUARE, X_RES, Y_RES};
use graphics::{ellipse, Color, Drawable, DrawingContext, Graphics};

pub struct Food {
    pub x: u64,
    pub y: u64,
    color: Color,
}

impl Food {
    pub fn new() -> Food {
        let x = (X_RES * 0.5 * rand::random::<f64>()).round() as u64;
        let y = (Y_RES * 0.5 * rand::random::<f64>()).round() as u64;
        Food {
            x,
            y,
            color: [1.0, 0.3, 0.0, 1.0],
        }
    }
}

impl Drawable for Food {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let x = (self.x as f64 + 0.1) * GRID_SQUARE;
        let y = (self.y as f64 + 0.1) * GRID_SQUARE;
        ellipse(
            self.color,
            [x, y, 0.8 * GRID_SQUARE, 0.8 * GRID_SQUARE],
            transform,
            gl,
        );
    }
}
