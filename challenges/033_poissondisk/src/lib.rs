use graphics::{
    colors::rgba, colors::Rgba, ellipse, Drawable, DrawingContext, EventHandler, Graphics,
    InputContext, Runnable, SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston::{Button, ButtonState, Key};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const SAMPLE_SIZE: f64 = 20.0;
const MAX_TRIES: usize = 1000;
const MIN_DIST: f64 = 30.0;

#[derive(Clone, Copy)]
struct Sample {
    pos: Vec2D,
    color: Rgba,
}

pub struct PoissonDisk {
    samples: Vec<Sample>,
    active_list: Vec<usize>,
}

impl PoissonDisk {
    pub fn new() -> PoissonDisk {
        PoissonDisk {
            samples: vec![],
            active_list: vec![],
        }
    }

    pub fn init(&mut self, window_width: f64, window_height: f64) {
        self.samples.push(Sample {
            pos: Vec2D::new(window_width / 2.0, window_height / 2.0),
            color: Rgba::random(),
        });
        self.active_list.push(0);
    }

    fn pick_next(&mut self) {
        if self.active_list.is_empty() {
            return;
        }

        let next_ind = rand::random::<usize>() % self.active_list.len();
        let sample = self.samples[next_ind];

        for _ in 0..MAX_TRIES {
            let next_dist = rand_between(MIN_DIST, 2.0 * MIN_DIST);
            let mut next_pos = Vec2D::rand_unit();
            next_pos.set_abs(next_dist);
            next_pos += sample.pos;

            if self
                .samples
                .iter()
                .any(|sam| sam.pos.dist(&next_pos) < MIN_DIST)
            {
                continue;
            }

            self.samples.push(Sample {
                pos: next_pos,
                color: Rgba::random(),
            });
            self.active_list.push(self.samples.len() - 1);
            return;
        }

        self.active_list.remove(next_ind);
    }
}

impl Drawable for PoissonDisk {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for (ind, sample) in self.samples.iter().enumerate() {
            let color = if self.active_list.contains(&ind) {
                rgba::WHITE
            } else {
                sample.color
            };

            ellipse(
                color.into(),
                [
                    sample.pos.x - SAMPLE_SIZE / 2.0,
                    sample.pos.y - SAMPLE_SIZE / 2.0,
                    SAMPLE_SIZE,
                    SAMPLE_SIZE,
                ],
                transform,
                gl,
            );

            /*ellipse(
                rgba::WHITE.with_trans(50).into(),
                [
                    sample.pos.x - MIN_DIST / 2.0,
                    sample.pos.y - MIN_DIST / 2.0,
                    MIN_DIST,
                    MIN_DIST,
                ],
                transform,
                gl,
            );*/
        }
    }
}

impl Updatable for PoissonDisk {
    fn update(&mut self, _: &mut UpdateContext) {
        self.pick_next();
    }
}

impl EventHandler for PoissonDisk {
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
            Key::Space => {
                self.samples.clear();
                self.active_list.clear();
                self.init(ctx.window_width, ctx.window_height);
            }
            _ => (),
        }
    }
}

impl Runnable for PoissonDisk {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.init(ctx.window_width, ctx.window_height);
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Poisson Disk Sampling".to_owned(),
        }
    }
}
