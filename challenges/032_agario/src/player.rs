use super::{MIN_SIZE, PLAYER_ACC, SHRINK_RATE};
use graphics::{
    colors::Rgba, ellipse, Drawable, DrawingContext, Graphics, Updatable, UpdateContext,
};
use math::vec2d::Vec2D;

pub struct Player {
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub size: f64,
    pub color: Rgba,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2D::default(),
            vel: Vec2D::default(),
            size: MIN_SIZE,
            color: Rgba::random(),
        }
    }
}

impl Drawable for Player {
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

impl Updatable for Player {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos += self.vel * ctx.args.dt;
        let mut player_target = Vec2D::new(ctx.mouse_pos[0], ctx.mouse_pos[1]) - self.pos;
        player_target.set_abs(PLAYER_ACC);
        self.vel = player_target;
        self.size -= SHRINK_RATE * ctx.args.dt;
    }
}
