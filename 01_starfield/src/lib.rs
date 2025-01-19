use graphics::{circle_arc, line};
use graphics_lib::{
    app::App,
    drawable::{Drawable, TransformMatrix},
    input_handler::InputHandler,
    point::Point,
};
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use std::f64::consts::PI;

const WINDOW_HEIGHT: f64 = 700.0;
const WINDOW_WIDTH: f64 = 700.0;

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
    fn draw(&self, _: &RenderArgs, gl: &mut GlGraphics, transform: TransformMatrix) {
        let mut shortened = self.pos.clone();
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

    fn update(&mut self, args: &UpdateArgs) {
        self.pos.set_abs(self.pos.abs() + args.dt * RAY_SPEED);
    }
}

struct StarSpawner {
    stars: Vec<Star>,
    max_x: f64,
    max_y: f64,
    min_x: f64,
    min_y: f64,
}

impl StarSpawner {
    fn new(window_width: f64, window_height: f64) -> StarSpawner {
        StarSpawner {
            stars: vec![],
            min_x: -window_width / 2.0,
            max_x: window_width / 2.0,
            min_y: -window_height / 2.0,
            max_y: window_height / 2.0,
        }
    }
}

impl Drawable for StarSpawner {
    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics, transform: TransformMatrix) {
        for star in self.stars.iter() {
            star.draw(args, gl, transform);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut to_remove = vec![];
        for (ind, star) in self.stars.iter_mut().enumerate() {
            star.update(args);
            if star.pos.x > self.max_x
                || star.pos.x < self.min_x
                || star.pos.y > self.max_y
                || star.pos.y < self.min_y
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

impl InputHandler for StarSpawner {}

pub fn run() {
    let mut app = App::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let spawner = StarSpawner::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    app.add_object(spawner);
    app.run();
}
