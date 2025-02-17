use super::{BULLET_LEN, BULLET_SPEED, BULLET_THICK};
use graphics::{
    colors::Rgba, line, poly_outline::PolyOutline, Drawable, DrawingContext, Graphics, Updatable,
    UpdateContext,
};
use math::vec2d::Vec2D;

pub struct Bullet {
    pub pos: Vec2D,
    vel: Vec2D,
    color: Rgba,
}

impl Bullet {
    pub fn new(x: f64, y: f64, dir: Vec2D) -> Bullet {
        let mut vel = dir;
        vel.set_abs(BULLET_SPEED);
        Bullet {
            pos: Vec2D::new(x, y),
            vel,
            color: Rgba::random(),
        }
    }

    pub fn as_poly(&self) -> PolyOutline {
        let mut heading = self.vel;
        heading.set_abs(BULLET_LEN);
        let end_point = heading;
        PolyOutline::new(self.pos, vec![Vec2D::default(), end_point], self.color)
    }
}

impl Drawable for Bullet {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let mut heading = self.vel;
        heading.set_abs(BULLET_LEN);
        let end_point = self.pos + heading;
        line(
            self.color.into(),
            BULLET_THICK,
            [self.pos.x, self.pos.y, end_point.x, end_point.y],
            transform,
            gl,
        )
    }
}

impl Updatable for Bullet {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos += self.vel * ctx.args.dt;
    }
}
