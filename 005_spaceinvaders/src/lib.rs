use core::array;
use graphics::{
    ellipse, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 12;
const ROW_DIST: f64 = 30.0;
const COL_DIST: f64 = 30.0;
const ROW_OFFSET: f64 = 30.0;
const ALIEN_WIDTH: f64 = 20.0;
const ALIEN_HEIGHT: f64 = 30.0;
const ALIEN_XSPEED: f64 = 1.0;
const ALIEN_YSPEED: f64 = 10.0;
const PROJECTILE_VELOCITY: f64 = 1500.0;
const PROJECTILE_THICKNESS: f64 = 1.0;
const PROJECTILE_LEN: f64 = 15.0;
const SHIP_WIDTH: f64 = 20.0;
const SHIP_HEIGHT: f64 = 25.0;
const SHIP_ACCEL: f64 = 800.0;

mod alien;
mod alienship;
mod projectile;
mod spaceship;
use alienship::AlienShip;
use spaceship::SpaceShip;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

pub struct SpaceInvaders {
    ship: SpaceShip,
    background_stars: [Vec2D; 3000],
    aliens: AlienShip<NUM_ROWS, NUM_COLS>,
}

impl SpaceInvaders {
    pub fn new() -> SpaceInvaders {
        SpaceInvaders {
            ship: SpaceShip::new(),
            background_stars: array::from_fn(|_| Vec2D {
                x: rand::random::<f64>() * 2.0 * WIDTH,
                y: rand::random::<f64>() * 2.0 * HEIGHT,
            }),
            aliens: AlienShip::new(),
        }
    }
}

impl Updatable for SpaceInvaders {
    fn update(&mut self, ctx: &UpdateContext) {
        self.ship.update(ctx);
        self.aliens.update(ctx);
        let projectiles = &self.ship.projectiles;

        if self
            .aliens
            .alien_rows
            .iter()
            .all(|row| row.iter().all(|alien| alien.dead))
            || self.aliens.alien_rows[NUM_ROWS - 1][0].pos.y >= ctx.window_height - SHIP_HEIGHT
        {
            std::process::exit(0)
        }

        let mut to_remove = vec![];
        for (ind, projectile) in projectiles.iter().enumerate() {
            for row in self.aliens.alien_rows.iter_mut() {
                for alien in row.iter_mut() {
                    if !alien.dead && alien.check_collision(projectile.pos_x, projectile.dist_top) {
                        alien.dead = true;
                        to_remove.push(ind)
                    }
                }
            }
        }
        to_remove.sort();
        for ind in to_remove {
            self.ship.projectiles.remove(ind);
        }
    }
}

impl Drawable for SpaceInvaders {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for star in self.background_stars.iter() {
            ellipse(
                [1.0, 1.0, 1.0, 1.0],
                [star.x, star.y, 2.0, 2.0],
                transform,
                gl,
            );
        }

        self.ship.draw(ctx, gl);
        self.aliens.draw(ctx, gl);
    }
}

impl EventHandler for SpaceInvaders {
    fn handle_input(&mut self, args: &InputContext) {
        self.ship.handle_input(args)
    }
}

impl Runnable for SpaceInvaders {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Space Invaders".to_owned(),
        }
    }
}
