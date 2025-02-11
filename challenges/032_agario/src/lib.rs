use graphics::{
    clear, line, Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext,
    Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MIN_SIZE: f64 = 20.0;
const FOOD_SIZE: f64 = 10.0;
const PLAYER_ACC: f64 = 300.0;
const ENEMY_ACC: f64 = 100.0;
const GROWTH: f64 = 1.0;
const FOOD_SPAWN_RATE: f64 = 0.1;
const NUM_ENEMIES: usize = 10;
const EAT_THRESHOLD: f64 = 5.0;
const SHRINK_RATE: f64 = 1.0;
const STARVE_THRESHOLD: f64 = 1.0;

const BG_GRID_W: f64 = 30.0;
const BG_GRID_H: f64 = 30.0;
const BG_TRANS: f32 = 0.7;

mod enemy;
mod food;
mod player;
use enemy::Enemy;
use food::Food;
use player::Player;

pub struct Agario {
    player: Player,
    food: Vec<Food>,
    enemies: Vec<Enemy>,
}

impl Agario {
    pub fn new() -> Agario {
        Agario {
            player: Player::new(),
            food: vec![],
            enemies: Vec::with_capacity(NUM_ENEMIES),
        }
    }

    fn game_over(&self) {
        std::process::exit(0);
    }

    fn draw_bg(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        clear([1.0, 1.0, 1.0, 1.0], gl);
        for i in 0..(ctx.args.window_size[0] / BG_GRID_W).ceil() as usize {
            let line_x = i as f64 * BG_GRID_W;
            line(
                [0.0, 0.0, 0.0, BG_TRANS],
                1.0,
                [line_x, 0.0, line_x, ctx.args.window_size[1]],
                transform,
                gl,
            )
        }

        for j in 0..(ctx.args.window_size[1] / BG_GRID_H).ceil() as usize {
            let line_y = j as f64 * BG_GRID_H;
            line(
                [0.0, 0.0, 0.0, BG_TRANS],
                1.0,
                [0.0, line_y, ctx.args.window_size[0], line_y],
                transform,
                gl,
            );
        }
    }

    fn collision(pos1: &Vec2D, pos2: &Vec2D, size1: f64, size2: f64) -> bool {
        pos1.dist(pos2) < size1 / 2.0 + size2 / 2.0
    }

    fn spawn_food(&mut self, window_width: f64, window_height: f64) {
        if rand::random::<f64>() < FOOD_SPAWN_RATE {
            self.food.push(Food::new(window_width, window_height));
        }
    }

    fn eat_food(&mut self) {
        struct Eaten {
            food_ind: usize,
            eater_ind: Option<usize>,
        }

        let mut to_be_eaten = vec![];
        'food_loop: for (ind, food) in self.food.iter().enumerate() {
            if Self::collision(&food.pos, &self.player.pos, FOOD_SIZE, self.player.size) {
                to_be_eaten.push(Eaten {
                    food_ind: ind,
                    eater_ind: None,
                });
                continue 'food_loop;
            }
            for (enemy_ind, enemy) in self.enemies.iter().enumerate() {
                if Self::collision(&food.pos, &enemy.pos, FOOD_SIZE, enemy.size) {
                    to_be_eaten.push(Eaten {
                        food_ind: ind,
                        eater_ind: Some(enemy_ind),
                    });
                    continue 'food_loop;
                }
            }
        }

        to_be_eaten.reverse();
        for eat in to_be_eaten {
            self.food.remove(eat.food_ind);
            match eat.eater_ind {
                None => self.player.size += GROWTH,
                Some(ind) => self.enemies[ind].size += GROWTH,
            }
        }
    }

    fn player_eat(&mut self) {
        let mut eaten = vec![];
        for (ind, enemy) in self.enemies.iter().enumerate() {
            if Self::collision(&self.player.pos, &enemy.pos, self.player.size, enemy.size) {
                if self.player.size + EAT_THRESHOLD < enemy.size {
                    self.game_over()
                }
                if enemy.size + EAT_THRESHOLD < self.player.size {
                    eaten.push(ind);
                }
            }
        }

        eaten.reverse();
        for ind in eaten {
            self.player.size += self.enemies[ind].size;
            self.enemies.remove(ind);
        }
    }

    fn get_colliding_enemies(&self) -> Vec<(usize, usize)> {
        let mut colliding = vec![];
        for (ind, enemy) in self.enemies.iter().enumerate() {
            for other_ind in (ind + 1)..(self.enemies.len() - 1) {
                let other = self.enemies.get(other_ind).unwrap();
                if Self::collision(&enemy.pos, &other.pos, enemy.size, other.size)
                    && !colliding.contains(&(ind, other_ind))
                    && !colliding.contains(&(other_ind, ind))
                {
                    colliding.push((ind, other_ind));
                }
            }
        }
        colliding
    }

    fn eat_blobs(&mut self) {
        self.player_eat();
        let collisions = self.get_colliding_enemies();

        let mut eaten = vec![];
        for (fst, snd) in collisions {
            if self.enemies[fst].size > self.enemies[snd].size + EAT_THRESHOLD {
                eaten.push(snd);
                self.enemies[fst].size += self.enemies[snd].size;
            }

            if self.enemies[snd].size > self.enemies[fst].size + EAT_THRESHOLD {
                eaten.push(fst);
                self.enemies[snd].size += self.enemies[fst].size;
            }
        }

        eaten.reverse();
        for ind in eaten {
            self.enemies.remove(ind);
        }
    }

    fn closest_food(&self, pos: Vec2D) -> Vec2D {
        let mut min_dist = f64::INFINITY;
        let mut min_pos = Vec2D::default();
        for food in self.food.iter() {
            let dist = pos.dist(&food.pos);
            if dist < min_dist {
                min_pos = food.pos;
                min_dist = dist;
            }
        }
        min_pos
    }

    fn set_enemy_targets(&mut self) {
        for i in 0..self.enemies.len() {
            let closest = self.closest_food(self.enemies[i].pos);
            self.enemies[i].target = closest;
        }
    }

    fn starve(&mut self) {
        if self.player.size <= STARVE_THRESHOLD {
            self.game_over();
        }

        let mut starved = vec![];
        for (ind, enemy) in self.enemies.iter().enumerate() {
            if enemy.size <= STARVE_THRESHOLD {
                starved.push(ind);
            }
        }
        starved.reverse();
        for ind in starved {
            self.enemies.remove(ind);
        }
    }
}

impl Drawable for Agario {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        self.draw_bg(ctx, gl);
        for food in self.food.iter() {
            food.draw(ctx, gl)
        }
        for enemy in self.enemies.iter() {
            enemy.draw(ctx, gl)
        }
        self.player.draw(ctx, gl);
    }
}

impl Updatable for Agario {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.spawn_food(ctx.window_width, ctx.window_height);

        self.player.update(ctx);
        self.set_enemy_targets();
        for enemy in self.enemies.iter_mut() {
            enemy.update(ctx);
        }

        self.eat_food();
        self.eat_blobs();

        self.starve();

        if self.enemies.len() < NUM_ENEMIES {
            self.enemies
                .push(Enemy::new(ctx.window_width, ctx.window_height));
        }
    }
}

impl EventHandler for Agario {}

impl Runnable for Agario {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.player.pos = Vec2D::new(ctx.window_width / 2.0, ctx.window_height / 2.0);
        for _ in 0..NUM_ENEMIES {
            self.spawn_food(ctx.window_width, ctx.window_height);
            self.enemies
                .push(Enemy::new(ctx.window_width, ctx.window_height));
        }
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Agar IO".to_owned(),
        }
    }
}
