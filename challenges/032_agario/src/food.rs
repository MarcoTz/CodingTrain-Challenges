use super::FOOD_SIZE;
use graphics::{colors::Rgba, ellipse, Drawable, DrawingContext, Graphics};
use math::{rand_between, vec2d::Vec2D};

pub struct Food {
    pub pos: Vec2D,
    color: Rgba,
}

impl Food {
    pub fn new(window_width: f64, window_height: f64) -> Food {
        Food {
            pos: Vec2D::new(
                rand_between(FOOD_SIZE, window_width),
                rand_between(FOOD_SIZE, window_height),
            ),
            color: Rgba::random(),
        }
    }
}

impl Drawable for Food {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        ellipse(
            self.color.into(),
            [
                self.pos.x - FOOD_SIZE / 2.0,
                self.pos.y - FOOD_SIZE / 2.0,
                FOOD_SIZE,
                FOOD_SIZE,
            ],
            transform,
            gl,
        );
    }
}
