use super::{/*trace::Trace, */ STEP_SIZE, WALKER_SIZE};
use graphics::{
    colors::Rgba, ellipse, Drawable, DrawingContext, Graphics, Updatable, UpdateContext,
};
use math::vec2d::Vec2D;

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    pub fn possible(pos: &Vec2D, window_width: f64, window_height: f64) -> Vec<Dir> {
        let mut possible_dirs = vec![];
        if pos.x + STEP_SIZE < window_width - WALKER_SIZE {
            possible_dirs.push(Dir::Right);
        }
        if pos.x - STEP_SIZE > 0.0 {
            possible_dirs.push(Dir::Left);
        }
        if pos.y + STEP_SIZE < window_height - WALKER_SIZE {
            possible_dirs.push(Dir::Down);
        }
        if pos.y - STEP_SIZE > 0.0 {
            possible_dirs.push(Dir::Up)
        }
        possible_dirs
    }
}

pub struct Walker {
    pub pos: Vec2D,
    color: Rgba,
    //    pub trace: Trace,
}

impl Walker {
    pub fn new() -> Walker {
        let color = Rgba::random();
        Walker {
            pos: Vec2D::default(),
            color,
            //trace: Trace::new(color),
        }
    }

    fn walk(&mut self, window_width: f64, window_height: f64) {
        let possible = Dir::possible(&self.pos, window_width, window_height);
        let dir_ind = rand::random::<usize>() % possible.len();
        let mut new_pos = self.pos;
        match possible[dir_ind] {
            Dir::Left => new_pos.x -= STEP_SIZE,
            Dir::Right => new_pos.x += STEP_SIZE,
            Dir::Up => new_pos.y -= STEP_SIZE,
            Dir::Down => new_pos.y += STEP_SIZE,
        }
        //        self.trace.push(self.pos);
        self.pos = new_pos;
    }
}

impl Drawable for Walker {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();

        //        self.trace.draw(ctx, gl);

        ellipse(
            self.color.into(),
            [
                self.pos.x - WALKER_SIZE / 2.0,
                self.pos.y - WALKER_SIZE / 2.0,
                WALKER_SIZE,
                WALKER_SIZE,
            ],
            transform,
            gl,
        )
    }
}

impl Updatable for Walker {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.walk(ctx.window_width, ctx.window_height)
    }
}
