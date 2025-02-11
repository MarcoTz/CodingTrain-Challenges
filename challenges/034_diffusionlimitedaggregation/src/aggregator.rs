use super::{AGG_SIZE, WALKER_SIZE};
use graphics::{colors::Rgba, ellipse, Drawable, DrawingContext, Graphics};
use math::vec2d::Vec2D;

pub struct Aggregator {
    pub pos: Vec2D,
    pub color: Rgba,
    pub aggregated: Vec<Vec2D>,
}

impl Aggregator {
    pub fn new() -> Aggregator {
        Aggregator {
            pos: Vec2D::default(),
            color: Rgba::random(),
            aggregated: vec![],
        }
    }

    pub fn walker_hits(&self, walker_pos: &Vec2D) -> bool {
        let cmp_fun = |pos: &Vec2D| pos.dist(&walker_pos) < WALKER_SIZE / 2.0 + AGG_SIZE / 2.0;
        cmp_fun(&self.pos) || self.aggregated.iter().any(|pt| cmp_fun(pt))
    }
}

impl Drawable for Aggregator {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        ellipse(
            self.color.into(),
            [
                self.pos.x - AGG_SIZE / 2.0,
                self.pos.y - AGG_SIZE / 2.0,
                AGG_SIZE,
                AGG_SIZE,
            ],
            transform,
            gl,
        );
        for pt in self.aggregated.iter() {
            ellipse(
                self.color.into(),
                [
                    pt.x - AGG_SIZE / 2.0,
                    pt.y - AGG_SIZE / 2.0,
                    AGG_SIZE,
                    AGG_SIZE,
                ],
                transform,
                gl,
            );
        }
    }
}
