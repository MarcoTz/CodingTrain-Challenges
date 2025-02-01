use super::{ALIEN_HEIGHT, ALIEN_WIDTH, ALIEN_XSPEED};
use graphics::{polygon, Drawable, DrawingContext, Graphics, Updatable, UpdateContext};
use math::vec2d::Vec2D;

pub struct Alien {
    pub pos: Vec2D,
    pub x_speed: f64,
    pub dead: bool,
}

impl Alien {
    pub fn new(x: f64, y: f64) -> Alien {
        Alien {
            pos: Vec2D { x, y },
            x_speed: ALIEN_XSPEED,
            dead: false,
        }
    }

    pub fn check_collision(&self, x: f64, y: f64) -> bool {
        self.pos.x <= x
            && self.pos.x + ALIEN_WIDTH >= x
            && self.pos.y <= y
            && self.pos.y + ALIEN_HEIGHT >= y
    }
}

impl Drawable for Alien {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        if self.dead {
            return;
        }
        let transform = ctx.id_trans();
        polygon(
            [0.0, 1.0, 0.0, 1.0],
            &[
                [self.pos.x, self.pos.y],
                [self.pos.x + ALIEN_WIDTH, self.pos.y],
                [self.pos.x + ALIEN_WIDTH, self.pos.y + 0.8 * ALIEN_HEIGHT],
                [self.pos.x + ALIEN_WIDTH / 2.0, self.pos.y + ALIEN_HEIGHT],
                [self.pos.x, self.pos.y + 0.8 * ALIEN_HEIGHT],
            ],
            transform,
            gl,
        );
    }
}

impl Updatable for Alien {
    fn update(&mut self, _: &UpdateContext) {
        self.pos.x += self.x_speed;
    }
}
