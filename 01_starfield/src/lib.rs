use graphics::{circle_arc, line};
use graphics_lib::{point::Point, Drawable, DrawingContext, Runnable, Updatable, UpdateContext};
use opengl_graphics::GlGraphics;
use std::f64::consts::PI;
use window::Size;

const WINDOW_HEIGHT: f64 = 600.0;
const WINDOW_WIDTH: f64 = 600.0;
const LINE_THICKNESS: f64 = 0.5;
const MAX_TAIL_LENGTH: f64 = 200.0;
const MIN_TAIL_LENGTH: f64 = 50.0;
const RAY_SPEED: f64 = 500.0;
const SPAWN_RATE: f64 = 0.9;

struct Star {
    pos: Point,
    color: [f32; 4],
    len: f64,
}

impl Star {
    fn new() -> Star {
        let dir = rand::random::<f64>() * 2.0 * PI;
        Star {
            pos: Point::from_polar(1.0, dir),
            color: [rand::random(), rand::random(), rand::random(), 1.0],
            len: MIN_TAIL_LENGTH + (rand::random::<f64>() * (MAX_TAIL_LENGTH - MIN_TAIL_LENGTH)),
        }
    }
}

impl Drawable for Star {
    type DrawingArgs = ();
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics, _: &()) {
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
    type UpdateArgs = ();
    fn update(&mut self, ctx: &UpdateContext, _: &()) {
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
    type DrawingArgs = ();
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics, _: &()) {
        for star in self.stars.iter() {
            star.draw(ctx, gl, &());
        }
    }
}

impl Updatable for StarSpawner {
    type UpdateArgs = ();
    fn update(&mut self, ctx: &UpdateContext, _: &()) {
        let mut to_remove = vec![];
        for (ind, star) in self.stars.iter_mut().enumerate() {
            star.update(ctx, &());
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

impl Runnable for StarSpawner {
    type DrawingArgs = ();
    type UpdateArgs = ();
    type HandlerArgs = ();

    fn window_size(&self) -> Size {
        Size {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        }
    }

    fn to_draw(&self) -> Vec<(&dyn Drawable<DrawingArgs = ()>, &())> {
        vec![(self, &())]
    }

    fn to_update(&mut self) -> Vec<(&mut dyn Updatable<UpdateArgs = ()>, &())> {
        vec![(self, &())]
    }
}
