use super::{MAX_GAP, MIN_GAP, PIPE_SPEED, PIPE_W};
use graphics::{rectangle, Drawable, DrawingContext, Graphics, Updatable, UpdateContext};
use math::{rand_between, vec2d::Vec2D};

pub struct Pipe {
    pub pos_x: f64,
    gap_start: f64,
    gap_end: f64,
}

impl Pipe {
    pub fn new(window_width: f64, window_height: f64) -> Pipe {
        let gap_start = rand_between(MIN_GAP, window_height - 2.0 * MAX_GAP);
        Pipe {
            pos_x: window_width,
            gap_start,
            gap_end: gap_start + rand_between(MIN_GAP, MAX_GAP),
        }
    }

    pub fn collides(&self, pos: Vec2D) -> bool {
        pos.x >= self.pos_x
            && pos.x <= self.pos_x + PIPE_W
            && !(pos.y >= self.gap_start && pos.y <= self.gap_end)
    }
}

impl Drawable for Pipe {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [self.pos_x, 0.0, PIPE_W, self.gap_start],
            transform,
            gl,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [self.pos_x, self.gap_end, PIPE_W, ctx.args.window_size[1]],
            transform,
            gl,
        );
    }
}

impl Updatable for Pipe {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos_x -= ctx.args.dt * PIPE_SPEED;
    }
}
