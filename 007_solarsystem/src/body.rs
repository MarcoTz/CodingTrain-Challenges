use graphics::{
    ellipse, Color, Drawable, DrawingContext, Graphics, Transformed, Updatable, UpdateContext,
};
use math::vec2d::Vec2D;

pub struct Body {
    pub center: Vec2D,
    velocity: Vec2D,
    pub accel: Vec2D,
    color: Color,
    pub mass: f64,
}

impl Body {
    pub fn new(center: Vec2D, color: Color, mass: f64, velocity: Vec2D) -> Body {
        Body {
            center,
            color,
            mass,
            velocity,
            accel: Vec2D::default(),
        }
    }
}
impl Drawable for Body {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx
            .context
            .transform
            .trans(ctx.args.window_size[0] / 2.0, ctx.args.window_size[1] / 2.0)
            .trans(self.center.x, self.center.y);
        let radius = self.mass * 2.0;
        ellipse(
            self.color,
            [-radius, -radius, 2.0 * radius, 2.0 * radius],
            transform,
            gl,
        );

        /* line(
            [1.0, 0.0, 0.0, 1.0],
            2.0,
            [0.0, 0.0, self.velocity.x, self.velocity.y],
            transform,
            gl,
        );
        line(
            [0.0, 1.0, 0.0, 1.0],
            2.0,
            [0.0, 0.0, self.accel.x, self.accel.y],
            transform,
            gl,
        );*/
    }
}
impl Updatable for Body {
    fn update(&mut self, ctx: &UpdateContext) {
        self.center += self.velocity * ctx.args.dt;
        self.velocity += self.accel * ctx.args.dt;
    }
}
