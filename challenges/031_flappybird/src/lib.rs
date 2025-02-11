use graphics::{
    Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable, SetupContext,
    Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use piston::{Button, ButtonState, Key};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const BIRD_SIZE: f64 = 50.0;
const GRAVITY: f64 = 500.0;
const LIFT: f64 = 400.0;

const MIN_GAP: f64 = 100.0;
const MAX_GAP: f64 = 300.0;
const PIPE_W: f64 = 100.0;
const PIPE_SPEED: f64 = 300.0;
const PIPE_SPAWN: f64 = 2.0;

mod bird;
mod pipe;

use bird::Bird;
use pipe::Pipe;

pub struct FlappyBird {
    bird: Bird,
    pipes: Vec<Pipe>,
    last_pipe: f64,
}

impl FlappyBird {
    pub fn new() -> FlappyBird {
        FlappyBird {
            bird: Bird::new(),
            pipes: vec![],
            last_pipe: 0.0,
        }
    }

    pub fn game_over(&self, window_height: f64) -> bool {
        let mut bird_back_bot = self.bird.pos;
        bird_back_bot.y += BIRD_SIZE;

        let mut bird_front_top = self.bird.pos;
        bird_front_top.x += BIRD_SIZE;

        if bird_front_top.y > window_height {
            return true;
        }

        let mut bird_front_bot = bird_front_top;
        bird_front_bot.y += BIRD_SIZE;

        if bird_front_bot.y < 0.0 {
            return true;
        }

        for pipe in self.pipes.iter() {
            if pipe.collides(self.bird.pos)
                || pipe.collides(bird_front_top)
                || pipe.collides(bird_front_bot)
                || pipe.collides(bird_back_bot)
            {
                return true;
            }
        }
        false
    }
}

impl Drawable for FlappyBird {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        self.bird.draw(ctx, gl);
        for pipe in self.pipes.iter() {
            pipe.draw(ctx, gl)
        }
    }
}

impl Updatable for FlappyBird {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.last_pipe += ctx.args.dt;

        self.bird.update(ctx);
        let mut to_remove = vec![];
        for (ind, pipe) in self.pipes.iter_mut().enumerate() {
            pipe.update(ctx);
            if pipe.pos_x + PIPE_W < 0.0 {
                to_remove.push(ind);
            }
        }
        to_remove.reverse();
        for ind in to_remove {
            self.pipes.remove(ind);
        }

        if self.last_pipe > PIPE_SPAWN {
            self.pipes
                .push(Pipe::new(ctx.window_width, ctx.window_height));
            self.last_pipe = 0.0;
        }

        if self.game_over(ctx.window_height) {
            std::process::exit(0)
        }
    }
}

impl EventHandler for FlappyBird {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release {
            return;
        }
        let key = if let Button::Keyboard(key) = ctx.args.button {
            key
        } else {
            return;
        };

        match key {
            Key::Space => self.bird.vel.y = -LIFT,
            _ => (),
        }
    }
}

impl Runnable for FlappyBird {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.bird.pos = Vec2D::new(2.0 * BIRD_SIZE, ctx.window_height / 2.0);
        self.pipes
            .push(Pipe::new(ctx.window_width, ctx.window_height));
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Flappy Bird".to_owned(),
        }
    }
}
