use graphics::{
    polygon, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Transformed,
    Updatable, UpdateContext,
};
use piston::{Button, ButtonState, Key};

use super::{projectile::Projectile, SHIP_ACCEL, SHIP_HEIGHT, SHIP_WIDTH};

pub struct SpaceShip {
    center_dist: f64,
    velocity: f64,
    pub projectiles: Vec<Projectile>,
}

impl SpaceShip {
    pub fn new() -> SpaceShip {
        SpaceShip {
            center_dist: 0.0,
            velocity: 0.0,
            projectiles: vec![],
        }
    }

    pub fn shoot(&mut self, window_width: f64, window_height: f64) {
        let projectile = Projectile::new(window_width / 2.0 + self.center_dist, window_height);
        self.projectiles.push(projectile)
    }
}

impl Drawable for SpaceShip {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for projectile in self.projectiles.iter() {
            projectile.draw(ctx, gl);
        }

        let transform = ctx.context.transform.trans(
            ctx.args.window_size[0] / 2.0,
            ctx.args.window_size[1] - SHIP_HEIGHT,
        );

        polygon(
            [1.0, 0.1, 0.7, 1.0],
            &[
                [self.center_dist - SHIP_WIDTH / 2.0, SHIP_HEIGHT],
                [self.center_dist, 0.0],
                [self.center_dist + SHIP_WIDTH / 2.0, SHIP_HEIGHT],
            ],
            transform,
            gl,
        );
    }
}

impl Updatable for SpaceShip {
    fn update(&mut self, ctx: &mut UpdateContext) {
        let mut to_remove = vec![];
        for (ind, projectile) in self.projectiles.iter_mut().enumerate() {
            projectile.update(ctx);
            if projectile.dist_top <= 0.0 {
                to_remove.push(ind);
            }
        }

        to_remove.sort();
        for ind in to_remove {
            self.projectiles.remove(ind);
        }

        self.center_dist = (self.center_dist + self.velocity * ctx.args.dt)
            .min(ctx.window_width / 2.0 - SHIP_WIDTH / 2.0)
            .max(-ctx.window_width / 2.0 + SHIP_WIDTH / 2.0);
    }
}

impl EventHandler for SpaceShip {
    fn handle_input(&mut self, ctx: &InputContext) {
        let state = ctx.args.state;

        let key = if let Button::Keyboard(key) = ctx.args.button {
            key
        } else {
            return;
        };

        match key {
            Key::Right | Key::D | Key::Left | Key::A if state == ButtonState::Release => {
                self.velocity = 0.0
            }
            Key::Right | Key::D => self.velocity += SHIP_ACCEL,
            Key::Left | Key::A => self.velocity -= SHIP_ACCEL,
            Key::Space if state == ButtonState::Press => {
                self.shoot(ctx.window_width, ctx.window_height)
            }
            _ => return,
        }
    }
}
