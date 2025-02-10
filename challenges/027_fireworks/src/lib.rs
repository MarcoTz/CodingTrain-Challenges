use graphics::{
    line, rectangle, Color, Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const SPAWN_RATE: f64 = 20.0;
const MIN_SPEED: f64 = 150.0;
const MAX_SPEED: f64 = 300.0;
const MIN_LIFE: f64 = 1.0;
const MAX_LIFE: f64 = 3.0;
const ROCKET_SIZE: f64 = 5.0;
const EXPLODE_TIME: f64 = 0.4;
const SPREAD: f64 = 200.0;
const EXPLOSION_DUP_TIME: f64 = 0.2;
const NUM_EXPLOSION_DIRS: u64 = 15;
const DEBRIS_TIME: f64 = 0.3;
const GRAVITY: f64 = 20.0;
const PARTICLE_SIZE: f64 = 0.8;

struct Debris {
    pos: Vec2D,
    life_time: f64,
    color: Color,
}

impl Debris {
    fn new(pos: Vec2D, color: Color) -> Debris {
        Debris {
            pos,
            life_time: DEBRIS_TIME,
            color,
        }
    }
}

struct Explosion {
    pos: Vec2D,
    color: Color,
    size: f64,
    life_time: f64,
    dirs: Vec<Vec2D>,
    since_dupl: f64,
}

impl Explosion {
    fn new(pos: Vec2D, color: Color) -> Explosion {
        Explosion {
            pos,
            color,
            size: ROCKET_SIZE,
            life_time: EXPLODE_TIME,
            dirs: Self::rand_dirs(),
            since_dupl: 0.0,
        }
    }

    fn rand_dirs() -> Vec<Vec2D> {
        let mut dirs = vec![];
        for _ in 0..NUM_EXPLOSION_DIRS {
            dirs.push(Vec2D::rand_unit());
        }
        dirs
    }

    fn duplicate(&self) -> Explosion {
        Explosion {
            pos: self.pos,
            color: self.color,
            size: ROCKET_SIZE,
            life_time: self.life_time,
            dirs: Self::rand_dirs(),
            since_dupl: 0.0,
        }
    }

    fn fall(self) -> Vec<Debris> {
        let mut debris = vec![];
        for dir in self.dirs {
            let pos = self.pos + dir;
            debris.push(Debris::new(pos, self.color));
        }
        debris
    }
}

struct Rocket {
    pos: Vec2D,
    speed: f64,
    life_time: f64,
    color: Color,
}

impl Rocket {
    pub fn new(window_width: f64, window_height: f64) -> Rocket {
        Rocket {
            pos: Vec2D::new(rand_between(0.0, window_width), window_height),
            speed: rand_between(MIN_SPEED, MAX_SPEED),
            life_time: rand_between(MIN_LIFE, MAX_LIFE),
            color: [
                rand_between(0.0, 1.0) as f32,
                rand_between(0.0, 1.0) as f32,
                rand_between(0.0, 1.0) as f32,
                1.0,
            ],
        }
    }

    pub fn explode(self) -> Explosion {
        Explosion::new(self.pos, self.color)
    }
}

pub struct Fireworks {
    rockets: Vec<Rocket>,
    explosions: Vec<Explosion>,
    debris: Vec<Debris>,
}

impl Fireworks {
    pub fn new() -> Fireworks {
        Fireworks {
            rockets: vec![],
            explosions: vec![],
            debris: vec![],
        }
    }
}

impl Drawable for Fireworks {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for rocket in self.rockets.iter() {
            rocket.draw(ctx, gl)
        }

        for expl in self.explosions.iter() {
            expl.draw(ctx, gl)
        }

        for deb in self.debris.iter() {
            deb.draw(ctx, gl)
        }
    }
}

impl Drawable for Rocket {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();

        rectangle(
            self.color,
            [
                self.pos.x + ROCKET_SIZE / 2.0,
                self.pos.y + ROCKET_SIZE / 2.0,
                ROCKET_SIZE / 2.0,
                ROCKET_SIZE / 2.0,
            ],
            transform,
            gl,
        );
    }
}

impl Drawable for Explosion {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for dir in self.dirs.iter() {
            let start_x = self.pos.x + self.size * dir.x - PARTICLE_SIZE;
            let start_y = self.pos.y + self.size * dir.y - PARTICLE_SIZE;
            line(
                self.color,
                2.0 * PARTICLE_SIZE,
                [
                    start_x,
                    start_y,
                    start_x + PARTICLE_SIZE,
                    start_y + PARTICLE_SIZE,
                ],
                transform,
                gl,
            );
        }
    }
}

impl Drawable for Debris {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            self.color,
            [self.pos.x, self.pos.y, PARTICLE_SIZE, PARTICLE_SIZE],
            transform,
            gl,
        );
    }
}

impl Updatable for Fireworks {
    fn update(&mut self, ctx: &mut UpdateContext) {
        if rand_between(0.0, 100.0) > SPAWN_RATE {
            self.rockets
                .push(Rocket::new(ctx.window_width, ctx.window_height));
        }

        let mut to_remove = vec![];
        for (ind, rocket) in self.rockets.iter_mut().enumerate() {
            rocket.update(ctx);
            if rocket.life_time <= 0.0 {
                to_remove.push(ind);
            }
        }

        to_remove.reverse();
        for ind in to_remove {
            let rock = self.rockets.remove(ind);
            self.explosions.push(rock.explode());
        }

        to_remove = vec![];
        let mut new_expls = vec![];
        for (ind, expl) in self.explosions.iter_mut().enumerate() {
            expl.update(ctx);
            if expl.life_time <= 0.0 {
                to_remove.push(ind);
            }
            if expl.since_dupl >= EXPLOSION_DUP_TIME {
                new_expls.push(expl.duplicate());
                expl.since_dupl = 0.0;
            }
        }
        self.explosions.extend(new_expls);

        to_remove.reverse();
        for ind in to_remove {
            let exp = self.explosions.remove(ind);
            let debris = exp.fall();
            self.debris.extend(debris)
        }

        to_remove = vec![];
        for (ind, deb) in self.debris.iter_mut().enumerate() {
            deb.update(ctx);
            if deb.life_time <= 0.0 {
                to_remove.push(ind)
            }
        }

        to_remove.reverse();
        for ind in to_remove {
            self.debris.remove(ind);
        }
    }
}

impl Updatable for Rocket {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos.y -= self.speed * ctx.args.dt;
        self.life_time -= ctx.args.dt;
    }
}

impl Updatable for Explosion {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.size += SPREAD * ctx.args.dt;
        self.life_time -= ctx.args.dt;
        self.since_dupl += ctx.args.dt;
    }
}

impl Updatable for Debris {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos.y += GRAVITY * ctx.args.dt;
        self.life_time -= ctx.args.dt;
    }
}

impl EventHandler for Fireworks {}

impl Runnable for Fireworks {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Fireworks".to_owned(),
        }
    }
}
