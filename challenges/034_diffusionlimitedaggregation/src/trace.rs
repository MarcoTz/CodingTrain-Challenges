use super::WALKER_SIZE;
use graphics::{colors::Rgba, rectangle, Drawable, DrawingContext, Graphics};
use math::vec2d::Vec2D;

pub struct Trace {
    color: Rgba,
    points: Vec<Vec2D>,
}

/*impl Trace {
    pub fn new(color: Rgba) -> Trace {
        Trace {
            color,
            points: vec![],
        }
    }

    pub fn push(&mut self, pt: Vec2D) {
        self.points.push(pt)
    }
}*/

impl Drawable for Trace {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for pos in self.points.iter() {
            rectangle(
                self.color.with_trans(10).into(),
                [pos.x, pos.y, WALKER_SIZE, WALKER_SIZE],
                transform,
                gl,
            )
        }
    }
}
