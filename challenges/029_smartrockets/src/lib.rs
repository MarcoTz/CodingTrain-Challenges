use graphics::{
    rectangle, Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston_window::text::Text;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const STEERING_FORCE: f64 = 300.0;
const ROCKET_SIZE: f64 = 30.0;
const COLLISION_DAMP: f64 = 0.5;

const GENERATION_SIZE: usize = 100;
const GENERATION_TIME: f64 = 4.0;
const NUM_GENES: usize = 400;
const MUTATION_RATE: f64 = 0.4;
const NUM_SURVIVORS: usize = GENERATION_SIZE / 10;

const NUM_OBSTACLES: usize = 20;
const OBSTACLE_MIN_W: f64 = 50.0;
const OBSTACLE_MAX_W: f64 = 100.0;
const OBSTACLE_MIN_H: f64 = 10.0;
const OBSTACLE_MAX_H: f64 = 50.0;

mod genes;
mod rocket;
use genes::Genes;
use rocket::Rocket;

pub struct Obstacle {
    pos: Vec2D,
    width: f64,
    height: f64,
}

impl Obstacle {
    fn contains(&self, pos: &Vec2D) -> bool {
        pos.x >= self.pos.x
            && pos.x <= self.pos.x + self.width
            && pos.y >= self.pos.y
            && pos.y <= self.pos.y + self.height
    }
}

pub struct SmartRockets {
    rockets: Vec<Rocket>,
    target: Vec2D,
    gen_age: f64,
    obstacles: Vec<Obstacle>,
}

impl SmartRockets {
    pub fn new() -> SmartRockets {
        SmartRockets {
            rockets: Vec::with_capacity(GENERATION_SIZE),
            target: Vec2D::default(),
            gen_age: 0.0,
            obstacles: Vec::with_capacity(NUM_OBSTACLES),
        }
    }

    fn collisions(&self) -> Vec<usize> {
        let mut colliding = vec![];
        for (rocket_ind, rocket) in self.rockets.iter().enumerate() {
            for obstacle in self.obstacles.iter() {
                let mut vel_norm = rocket.pos;
                vel_norm.set_abs(1.0);

                let corners = rocket.corners();
                if corners.iter().any(|pos| obstacle.contains(&pos)) {
                    colliding.push(rocket_ind);
                }
            }
        }
        colliding
    }

    fn scores(&self) -> Vec<f64> {
        let mut scores = vec![];
        for rocket in self.rockets.iter() {
            scores.push(1.0 / rocket.pos.dist(&self.target));
        }
        scores
    }

    pub fn select_next_genes(&self) -> Vec<Genes> {
        let scores = self.scores();
        println!(
            "max score:{}",
            scores
                .iter()
                .max_by(|sc1, sc2| sc1.partial_cmp(sc2).unwrap())
                .unwrap()
        );

        let mut genes: Vec<(Genes, f64)> = self
            .rockets
            .iter()
            .map(|rocket| rocket.genes.clone())
            .zip(scores)
            .collect();
        genes.sort_by(|(_, sc1), (_, sc2)| sc2.partial_cmp(sc1).unwrap());
        genes
            .into_iter()
            .map(|(gene, _)| gene)
            .take(NUM_SURVIVORS)
            .collect()
    }

    pub fn crossover(parents: Vec<Genes>) -> Vec<Genes> {
        let num_parents = parents.len();
        let mut new_genes = vec![];
        for _ in 0..GENERATION_SIZE {
            let parent1_ind = rand::random::<usize>() % num_parents;
            let parent2_ind = rand::random::<usize>() % num_parents;
            let parent1 = parents.get(parent1_ind).unwrap();
            let parent2 = parents.get(parent2_ind).unwrap();
            new_genes.push(parent1.cross(parent2));
        }
        new_genes
    }

    pub fn first_gen(&mut self, window_width: f64, window_height: f64) {
        for _ in 0..GENERATION_SIZE {
            self.rockets
                .push(Rocket::new(window_width / 2.0, window_height - ROCKET_SIZE))
        }
    }

    pub fn next_gen(&mut self, window_width: f64, window_height: f64) {
        let parent_genes = self.select_next_genes();
        let next_genes = Self::crossover(parent_genes);

        self.gen_age = 0.0;
        self.rockets.clear();
        for mut next_gene in next_genes {
            next_gene.mutate();
            self.rockets.push(Rocket::with_genes(
                window_width / 2.0,
                window_height - ROCKET_SIZE,
                next_gene,
            ));
        }
    }
}

impl Drawable for SmartRockets {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            [1.0, 0.0, 0.0, 1.0],
            [self.target.x, self.target.y, 10.0, 10.0],
            transform,
            gl,
        );
        Text::new_color([1.0, 1.0, 1.0, 1.0], 18)
            .draw_pos(
                &format!("{:.2}", self.gen_age),
                [0.0, 18.0],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();

        for rocket in self.rockets.iter() {
            rocket.draw(ctx, gl);
        }

        for obstacle in self.obstacles.iter() {
            obstacle.draw(ctx, gl);
        }
    }
}

impl Drawable for Obstacle {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            [0.0, 0.0, 1.0, 1.0],
            [self.pos.x, self.pos.y, self.width, self.height],
            transform,
            gl,
        );
    }
}

impl Updatable for SmartRockets {
    fn update(&mut self, ctx: &mut UpdateContext) {
        for rocket in self.rockets.iter_mut() {
            rocket.update(ctx);
        }

        for rocket_ind in self.collisions() {
            let rocket = self.rockets.get_mut(rocket_ind).unwrap();
            rocket.pos -= COLLISION_DAMP * rocket.vel;
            rocket.vel *= -COLLISION_DAMP;
        }

        self.gen_age += ctx.args.dt;
        if self.gen_age > GENERATION_TIME {
            self.next_gen(ctx.window_width, ctx.window_height);
        }
    }
}

impl EventHandler for SmartRockets {}

impl Runnable for SmartRockets {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.target = Vec2D::new(
            rand_between(0.0, ctx.window_width),
            rand_between(0.0, ctx.window_width),
        );
        self.first_gen(ctx.window_width, ctx.window_height);

        while self.obstacles.len() < NUM_OBSTACLES {
            let new_pos = Vec2D::new(
                rand_between(0.0, ctx.window_width),
                rand_between(0.0, ctx.window_height),
            );
            let new_w = rand_between(OBSTACLE_MIN_W, OBSTACLE_MAX_W);
            let new_h = rand_between(OBSTACLE_MIN_H, OBSTACLE_MAX_H);
            let top_right = new_pos + Vec2D::new(new_w, 0.0);
            let bottom_left = new_pos + Vec2D::new(0.0, new_h);
            let bottom_right = new_pos + Vec2D::new(new_w, new_h);

            if self.obstacles.iter().any(|obs| {
                obs.contains(&new_pos)
                    || obs.contains(&top_right)
                    || obs.contains(&bottom_left)
                    || obs.contains(&bottom_right)
            }) {
                continue;
            }
            self.obstacles.push(Obstacle {
                pos: new_pos,
                width: new_w,
                height: new_h,
            });
        }
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Smart Rockets".to_owned(),
        }
    }
}
