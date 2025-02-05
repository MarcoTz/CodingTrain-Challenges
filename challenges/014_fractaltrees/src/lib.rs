use graphics::{
    line, Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext, Transformed,
    Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use std::f64::consts::PI;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const START_LEN: f64 = 300.0;
const NUM_SPLIT: usize = 3;
const SHORTEN: f64 = 0.5;
const ANGLE_OFFSET: f64 = -PI / 2.0;
const ANGLE_DIFF: f64 = PI / (NUM_SPLIT as f64);

#[derive(Debug)]
pub struct FractalTree {
    current_len: f64,
    max_len: f64,
    bottom: Vec2D,
    angle: f64,
    next: Vec<FractalTree>,
    is_split: bool,
}

impl FractalTree {
    pub fn new() -> FractalTree {
        FractalTree {
            current_len: 0.0,
            max_len: START_LEN,
            next: Vec::with_capacity(NUM_SPLIT),
            bottom: Vec2D::default(),
            angle: 0.0,
            is_split: false,
        }
    }

    pub fn end(&self) -> Vec2D {
        self.bottom + Vec2D::from_polar(self.max_len, self.angle + ANGLE_OFFSET)
    }

    fn split(&mut self) {
        self.is_split = true;
        if self.max_len <= 1.0 {
            return;
        }
        let mut next_angle = ANGLE_OFFSET + ANGLE_DIFF;
        for _ in 0..NUM_SPLIT {
            let mut tree = FractalTree::new();
            tree.max_len = self.max_len * SHORTEN;
            tree.bottom = self.end();
            tree.angle = next_angle;
            self.next.push(tree);
            next_angle += 2.0 * ANGLE_DIFF;
        }
    }
}

impl Drawable for FractalTree {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx
            .context
            .transform
            .trans(self.bottom.x, self.bottom.y)
            .rot_rad(self.angle);
        line(
            [1.0, 1.0, 1.0, 1.0],
            2.0,
            [0.0, 0.0, 0.0, -self.current_len],
            transform,
            gl,
        );
        for tree in self.next.iter() {
            tree.draw(ctx, gl);
        }
    }
}

impl Updatable for FractalTree {
    fn update(&mut self, ctx: &UpdateContext) {
        if self.current_len < self.max_len {
            self.current_len += ctx.args.dt * ctx.window_height / 10.0;
        } else if !self.is_split {
            self.split();
        }
        for t in self.next.iter_mut() {
            t.update(ctx)
        }
    }
}

impl EventHandler for FractalTree {}

impl Runnable for FractalTree {
    fn setup(&mut self, ctx: &SetupContext) {
        self.bottom = Vec2D::new(ctx.window_width / 2.0, ctx.window_height);
    }
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Fractal Trees".to_owned(),
        }
    }
}
