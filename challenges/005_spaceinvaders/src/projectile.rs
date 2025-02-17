use super::{PROJECTILE_LEN, PROJECTILE_THICKNESS, PROJECTILE_VELOCITY};
use graphics::{line, Drawable, DrawingContext, Graphics, Updatable, UpdateContext};

#[derive(Debug)]
pub struct Projectile {
    pub dist_top: f64,
    pub pos_x: f64,
}

impl Projectile {
    pub fn new(x: f64, y: f64) -> Projectile {
        Projectile {
            dist_top: y,
            pos_x: x,
        }
    }
}

impl Drawable for Projectile {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let trans = ctx.id_trans();
        line(
            [1.0, 1.0, 1.0, 1.0],
            PROJECTILE_THICKNESS,
            [
                self.pos_x,
                self.dist_top,
                self.pos_x,
                self.dist_top + PROJECTILE_LEN,
            ],
            trans,
            gl,
        )
    }
}

impl Updatable for Projectile {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.dist_top -= PROJECTILE_VELOCITY * ctx.args.dt;
    }
}
