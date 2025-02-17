use graphics::{
    ellipse, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston::{Button, ButtonState, Key};
use std::f64::consts::PI;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const PLAYER_SIZE: f64 = 40.0;
const LINE_THICK: f64 = 2.0;
const PLAYER_ROT: f64 = 2.0 * PI;
const PLAYER_ACC: f64 = 3000.0;
const FRICTION: f64 = 0.9;

const BULLET_LEN: f64 = 20.0;
const BULLET_THICK: f64 = 2.0;
const BULLET_SPEED: f64 = 1500.0;

const ASTEROID_SPEED: f64 = 100.0;
const AST_MIN_BASE_R: f64 = 30.0;
const AST_MAX_BASE_R: f64 = 100.0;
const AST_MIN_R_OFF: f64 = 10.0;
const AST_MAX_R_OFF: f64 = 50.0;
const AST_MIN_VERTS: usize = 4;
const AST_MAX_VERTS: usize = 10;
const AST_SPAWN_TIME: f64 = 1.0;
const AST_MIN_SPLIT: f64 = 50.0;

const NUM_BG: usize = 200;

mod asteroid;
mod bullet;
mod player;
use asteroid::Asteroid;
use bullet::Bullet;
use player::Player;

pub struct Asteroids {
    player: Player,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    ticks: f64,
    background: Vec<Vec2D>,
}

impl Asteroids {
    pub fn new() -> Asteroids {
        Asteroids {
            player: Player::new(),
            bullets: vec![],
            asteroids: vec![],
            ticks: 0.0,
            background: vec![],
        }
    }

    pub fn check_bullets(&mut self, ctx: &mut UpdateContext) {
        let mut to_remove = vec![];
        for (ind, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update(ctx);
            if bullet.pos.x <= 0.0
                || bullet.pos.x >= ctx.window_width
                || bullet.pos.y <= 0.0
                || bullet.pos.y >= ctx.window_height
            {
                to_remove.push(ind);
            }
        }

        to_remove.reverse();
        for ind in to_remove {
            self.bullets.remove(ind);
        }
    }

    pub fn spawn_asteroid(&mut self, window_width: f64, window_height: f64) {
        if self.ticks > AST_SPAWN_TIME {
            self.asteroids
                .push(Asteroid::new(window_width, window_height));
            self.ticks = 0.0;
        }
    }

    pub fn collisions_bullets_asteroids(&mut self) {
        let mut hit = vec![];
        let mut used_bullets = vec![];
        'bullet_loop: for (bullet_ind, bullet) in self.bullets.iter().enumerate() {
            let bullet_poly = bullet.as_poly();
            for (ast_ind, asteroid) in self.asteroids.iter().enumerate() {
                if bullet_poly.collides(&asteroid.shape) {
                    hit.push(ast_ind);
                    used_bullets.push(bullet_ind);
                    continue 'bullet_loop;
                }
            }
        }

        used_bullets.reverse();
        for ind in used_bullets {
            self.bullets.remove(ind);
        }

        hit.sort();
        for ind in hit {
            let ast = self.asteroids.remove(ind);
            match ast.split() {
                None => (),
                Some(asts) => self.asteroids.extend(asts),
            };
        }
    }
}

impl Drawable for Asteroids {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for bg_star in self.background.iter() {
            ellipse(
                [1.0, 1.0, 1.0, 1.0],
                [bg_star.x, bg_star.y, 1.0, 1.0],
                transform,
                gl,
            );
        }

        self.player.draw(ctx, gl);
        for bullet in self.bullets.iter() {
            bullet.draw(ctx, gl)
        }
        for asteroid in self.asteroids.iter() {
            asteroid.draw(ctx, gl)
        }
    }
}

impl Updatable for Asteroids {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.ticks += ctx.args.dt;
        self.spawn_asteroid(ctx.window_width, ctx.window_height);
        self.player.update(ctx);
        self.check_bullets(ctx);

        for asteroid in self.asteroids.iter_mut() {
            asteroid.update(ctx);
        }

        if self
            .asteroids
            .iter()
            .any(|ast| self.player.shape.collides(&ast.shape))
        {
            std::process::exit(1)
        }

        self.collisions_bullets_asteroids();
    }
}

impl EventHandler for Asteroids {
    fn handle_input(&mut self, ctx: &InputContext) {
        self.player.handle_input(ctx);
        if ctx.args.state == ButtonState::Release && ctx.args.button == Button::Keyboard(Key::Space)
        {
            self.bullets.push(self.player.shoot());
        }
    }
}

impl Runnable for Asteroids {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.player.center(ctx.window_width, ctx.window_height);
        self.asteroids
            .push(Asteroid::new(ctx.window_width, ctx.window_height));
        for _ in 0..NUM_BG {
            self.background.push(Vec2D::new(
                rand_between(0.0, ctx.window_width),
                rand_between(0.0, ctx.window_height),
            ))
        }
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Asteroids".to_owned(),
        }
    }
}
