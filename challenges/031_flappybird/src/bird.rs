use super::{BIRD_SIZE, GRAVITY};
use graphics::{rectangle, Drawable, DrawingContext, Graphics, Updatable, UpdateContext};
use math::vec2d::Vec2D;

pub struct Bird {
    pub pos: Vec2D,
    pub vel: Vec2D,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            pos: Vec2D::default(),
            vel: Vec2D::new(0.0, GRAVITY),
        }
    }
}

impl Drawable for Bird {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [self.pos.x, self.pos.y, BIRD_SIZE, BIRD_SIZE],
            transform,
            gl,
        )
    }
}

impl Updatable for Bird {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos += self.vel * ctx.args.dt;
        if self.vel.y < GRAVITY {
            self.vel.y += GRAVITY * ctx.args.dt;
        }
    }
}
