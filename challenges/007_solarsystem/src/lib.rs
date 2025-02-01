use graphics::{
    ellipse, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston::{Button, ButtonState, MouseButton, ResizeArgs};
use std::iter;

mod body;
use body::Body;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MAX_R: f64 = 10.0;
const MIN_R: f64 = 1.0;
const GRAVITY: f64 = 100.0;

pub struct SolarSystem {
    sun: Body,
    bodies: Vec<Body>,
    background_stars: Vec<Vec2D>,
}

impl SolarSystem {
    pub fn new() -> SolarSystem {
        SolarSystem {
            sun: Body::new(
                Vec2D::default(),
                [1.0, 0.8, 0.0, 1.0],
                MAX_R * 10.0,
                Vec2D::default(),
            ),
            bodies: vec![],
            background_stars: vec![],
        }
    }

    pub fn calculate_gravity(&self) -> Vec<Vec2D> {
        let mut forces: Vec<Vec2D> = iter::repeat(Vec2D::default())
            .take(self.bodies.len())
            .collect();

        for ind1 in 0..self.bodies.len() {
            let fst = &self.bodies[ind1];
            let abs = fst.center.abs();
            let sun_gravity = GRAVITY * (fst.mass * self.sun.mass) / (abs * abs);
            forces[ind1] -= fst.center * sun_gravity;
            for ind2 in (ind1 + 1)..self.bodies.len() {
                let snd = &self.bodies[ind2];

                let dist = fst.center.dist(&snd.center);
                if dist < fst.mass + snd.mass {
                    continue;
                }
                let force = (GRAVITY * (fst.mass * snd.mass)) / (dist * dist);
                let mut direction = fst.center - snd.center;
                direction.set_abs(force);
                forces[ind1] -= direction;
                forces[ind2] += direction;
            }
        }
        forces
    }

    fn generate_background(&mut self, window_width: f64, window_height: f64) {
        self.background_stars = vec![];
        for _ in 1..1000 {
            self.background_stars.push(Vec2D::new(
                rand_between(-window_width / 2.0, window_width / 2.0),
                rand_between(-window_height / 2.0, window_height / 2.0),
            ));
        }
    }
}

impl Drawable for SolarSystem {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx.center_trans();
        for star in self.background_stars.iter() {
            ellipse(
                [1.0, 1.0, 1.0, 1.0],
                [star.x, star.y, 2.0, 2.0],
                transform,
                gl,
            );
        }

        self.sun.draw(ctx, gl);
        for body in self.bodies.iter() {
            body.draw(ctx, gl);
        }
    }
}

impl Updatable for SolarSystem {
    fn update(&mut self, ctx: &UpdateContext) {
        for body in self.bodies.iter_mut() {
            body.update(ctx);
        }

        let forces = self.calculate_gravity();

        for (ind, body) in self.bodies.iter_mut().enumerate() {
            body.accel = forces[ind] / body.mass;

            if body.center.x <= body.mass
                || body.center.x >= ctx.window_width - body.mass
                || body.center.y <= body.mass
                || body.center.y >= ctx.window_height - body.mass
            {}
        }
    }
}

impl EventHandler for SolarSystem {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release
            || ctx.args.button != Button::Mouse(MouseButton::Left)
        {
            return;
        }

        let new_pos = Vec2D {
            x: ctx.mouse_pos[0] - (ctx.window_width / 2.0),
            y: ctx.mouse_pos[1] - (ctx.window_height / 2.0),
        };
        let abs = new_pos.abs();
        let mass = rand_between(MIN_R, MAX_R);
        let escape_velocity =
            (2.0 * GRAVITY * self.sun.mass / (abs - self.sun.mass).abs().min(1.0)).sqrt();
        self.bodies.push(Body::new(
            new_pos,
            [rand::random(), rand::random(), rand::random(), 1.0],
            mass,
            new_pos.tangent() * escape_velocity,
        ));
    }

    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.generate_background(ctx.window_size[0], ctx.window_size[1]);
    }
}

impl Runnable for SolarSystem {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Solar System".to_owned(),
        }
    }

    fn setup(&mut self, ctx: &SetupContext) {
        self.generate_background(ctx.window_width, ctx.window_height);
    }
}
