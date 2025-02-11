use super::{ENEMY_ACC, MIN_SIZE, SHRINK_RATE};
use graphics::{
    colors::Rgba, ellipse, Drawable, DrawingContext, Graphics, Updatable, UpdateContext,
};
use math::{rand_between, vec2d::Vec2D};

pub struct Enemy {
    pub pos: Vec2D,
    vel: Vec2D,
    pub target: Vec2D,
    pub size: f64,
    color: Rgba,
}

impl Enemy {
    pub fn new(window_width: f64, window_height: f64) -> Enemy {
        let pos = Vec2D::new(
            rand_between(0.0, window_width),
            rand_between(0.0, window_height),
        );
        Enemy {
            pos,
            vel: Vec2D::default(),
            target: pos,
            size: MIN_SIZE,
            color: Rgba::random(),
        }
    }
}

impl Drawable for Enemy {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        ellipse(
            self.color.into(),
            [
                self.pos.x - self.size / 2.0,
                self.pos.y - self.size / 2.0,
                self.size,
                self.size,
            ],
            transform,
            gl,
        )
    }
}

impl Updatable for Enemy {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos += self.vel * ctx.args.dt;
        let mut target = self.target - self.pos;
        target.set_abs(ENEMY_ACC);
        self.vel = target;
        self.size -= SHRINK_RATE * ctx.args.dt;
    }
}
