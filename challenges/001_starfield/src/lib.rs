use graphics::{
    circle_arc, line, Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use std::f64::consts::PI;

const WINDOW_HEIGHT: f64 = 600.0;
const WINDOW_WIDTH: f64 = 600.0;
const LINE_THICKNESS: f64 = 0.5;
const MAX_TAIL_LENGTH: f64 = 200.0;
const MIN_TAIL_LENGTH: f64 = 50.0;
const RAY_SPEED: f64 = 500.0;
const SPAWN_RATE: f64 = 0.9;

struct Star {
    pos: Vec2D,
    color: [f32; 4],
    len: f64,
}

impl Star {
    fn new() -> Star {
        let dir = rand::random::<f64>() * 2.0 * PI;
        Star {
            pos: Vec2D::from_polar(1.0, dir),
            color: [rand::random(), rand::random(), rand::random(), 1.0],
            len: MIN_TAIL_LENGTH + (rand::random::<f64>() * (MAX_TAIL_LENGTH - MIN_TAIL_LENGTH)),
        }
    }
}

impl Drawable for Star {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx.center_trans();
        let mut shortened = self.pos;
        shortened.set_abs(self.pos.abs() - self.len);

        if shortened.abs() < self.len {
            return;
        }

        line(
            self.color,
            LINE_THICKNESS,
            [self.pos.x, self.pos.y, shortened.x, shortened.y],
            transform,
            gl,
        );
        let circle_radius = 5.0 * LINE_THICKNESS;
        circle_arc(
            self.color,
            circle_radius,
            0.0,
            2.0 * PI,
            [
                self.pos.x - circle_radius / 2.0,
                self.pos.y - circle_radius / 2.0,
                circle_radius / 2.0,
                circle_radius / 2.0,
            ],
            transform,
            gl,
        );
    }
}

impl Updatable for Star {
    fn update(&mut self, ctx: &UpdateContext) {
        self.pos.set_abs(self.pos.abs() + ctx.args.dt * RAY_SPEED);
    }
}

pub struct StarSpawner {
    stars: Vec<Star>,
}

impl StarSpawner {
    pub fn new() -> StarSpawner {
        StarSpawner { stars: vec![] }
    }
}

impl Default for StarSpawner {
    fn default() -> StarSpawner {
        StarSpawner::new()
    }
}

impl Drawable for StarSpawner {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        for star in self.stars.iter() {
            star.draw(ctx, gl);
        }
    }
}

impl Updatable for StarSpawner {
    fn update(&mut self, ctx: &UpdateContext) {
        let mut to_remove = vec![];
        for (ind, star) in self.stars.iter_mut().enumerate() {
            star.update(ctx);
            if star.pos.x > ctx.window_width / 2.0
                || star.pos.x < -ctx.window_width / 2.0
                || star.pos.y > ctx.window_height / 2.0
                || star.pos.y < -ctx.window_height / 2.0
            {
                to_remove.push(ind)
            }
        }
        to_remove.sort();
        for ind in to_remove.into_iter().rev() {
            self.stars.remove(ind);
        }

        if rand::random::<f64>() <= SPAWN_RATE {
            self.stars.push(Star::new());
        }
    }
}

impl EventHandler for StarSpawner {}

impl Runnable for StarSpawner {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Starfield".to_owned(),
        }
    }
}
