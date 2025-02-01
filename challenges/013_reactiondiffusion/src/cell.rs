use super::{
    interpolate_color, DIFFUSION_A, DIFFUSION_B, FEED_RATE, KILL_RATE, NUM_COLS, NUM_ROWS,
};
use graphics::{rectangle, Drawable, DrawingContext, Graphics, Updatable, UpdateContext};

#[derive(Clone, Copy)]
pub struct Cell {
    pub width: f64,
    pub height: f64,
    pub x: u64,
    pub y: u64,
    pub concentration_a: f64,
    pub concentration_b: f64,
    pub laplace_a: f64,
    pub laplace_b: f64,
}

impl Cell {
    pub fn new(x: u64, y: u64) -> Cell {
        Cell {
            width: 0.0,
            height: 0.0,
            x,
            y,
            concentration_a: 1.0,
            concentration_b: 0.0,
            laplace_a: 0.0,
            laplace_b: 0.0,
        }
    }
}

impl Drawable for Cell {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let row_y = ctx.args.window_size[1] / NUM_ROWS as f64;
        let col_x = ctx.args.window_size[0] / NUM_COLS as f64;
        let color = interpolate_color(self.concentration_a as f32, self.concentration_b as f32);
        rectangle(
            color,
            [
                col_x * self.x as f64,
                row_y * self.y as f64,
                self.width,
                self.height,
            ],
            transform,
            gl,
        );
    }
}

impl Updatable for Cell {
    fn update(&mut self, _: &UpdateContext) {
        let reaction = self.concentration_a * self.concentration_b * self.concentration_b;
        self.concentration_a = self.concentration_a
            + (DIFFUSION_A * self.laplace_a - reaction + FEED_RATE * (1.0 - self.concentration_a));
        self.concentration_b = self.concentration_b
            + (DIFFUSION_B * self.laplace_b + reaction
                - (KILL_RATE + FEED_RATE) * self.concentration_b);
    }
}
