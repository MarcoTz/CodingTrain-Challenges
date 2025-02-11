use super::{genes::Genes, GENERATION_TIME, NUM_GENES, ROCKET_SIZE};
use graphics::{polygon, Drawable, DrawingContext, Graphics, Updatable, UpdateContext};
use math::vec2d::Vec2D;
use std::f64::consts::PI;

pub struct Rocket {
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub accel: Vec2D,
    pub genes: Genes,
    pub next_gene: f64,
}

impl Rocket {
    pub fn new(x: f64, y: f64) -> Rocket {
        Rocket {
            pos: Vec2D::new(x, y),
            vel: Vec2D::default(),
            accel: Vec2D::default(),
            genes: Genes::new(),
            next_gene: 0.0,
        }
    }

    pub fn with_genes(x: f64, y: f64, genes: Genes) -> Rocket {
        Rocket {
            pos: Vec2D::new(x, y),
            vel: Vec2D::default(),
            accel: Vec2D::default(),
            genes,
            next_gene: 0.0,
        }
    }

    pub fn corners(&self) -> Vec<Vec2D> {
        let mut bottom_left = -self.vel;
        bottom_left.set_abs(ROCKET_SIZE);
        bottom_left.set_arg(bottom_left.arg() - PI / 8.0);
        bottom_left += self.pos;

        let mut bottom_right = -self.vel;
        bottom_right.set_abs(ROCKET_SIZE);
        bottom_right.set_arg(bottom_right.arg() + PI / 8.0);
        bottom_right += self.pos;

        vec![self.pos, bottom_left, bottom_right]
    }
}

impl Drawable for Rocket {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();

        let corners = self.corners();
        polygon(
            [1.0, 1.0, 1.0, 1.0],
            &[
                [corners[0].x, corners[0].y],
                [corners[1].x, corners[1].y],
                [corners[2].x, corners[2].y],
            ],
            transform,
            gl,
        )
    }
}

impl Updatable for Rocket {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.next_gene += ctx.args.dt;
        self.pos += self.vel * ctx.args.dt;
        self.vel += self.accel * ctx.args.dt;
        if self.next_gene >= GENERATION_TIME / NUM_GENES as f64 {
            self.accel = self.genes.next().unwrap();
            self.next_gene = 0.0;
        }
    }
}
