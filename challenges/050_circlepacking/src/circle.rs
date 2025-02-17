use graphics::{colors::Rgba, ellipse, Drawable, DrawingContext, Graphics, Transformed};
use math::vec2d::Vec2D;

pub struct Circle {
    pub pos: Vec2D,
    radius: f64,
    color: Rgba,
}

impl Circle {
    pub fn new(pos: Vec2D, r: f64) -> Circle {
        Circle {
            pos,
            radius: r,
            color: Rgba::random(),
        }
    }

    pub fn collides(&self, other_pos: &Vec2D, other_r: f64) -> bool {
        self.pos.dist(&other_pos) < self.radius + other_r
    }
}

impl Drawable for Circle {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.context.transform.trans(self.pos.x, self.pos.y);
        ellipse(
            self.color.into(),
            [
                -self.radius,
                -self.radius,
                2.0 * self.radius,
                2.0 * self.radius,
            ],
            transform,
            gl,
        );
    }
}
