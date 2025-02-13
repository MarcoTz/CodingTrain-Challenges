use graphics::{
    colors::Rgba, ellipse, line, Drawable, DrawingContext, EventHandler, Graphics, Runnable,
    SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston_window::text::Text;
use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILE: &str = "challenges/042_markovchain/text.txt";

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const STATE_SIZE: f64 = 20.0;
const LINE_SIZE: f64 = 2.0;
const UPDATE_TIME: f64 = 0.1;

struct MarkovState {
    output: String,
    color: Rgba,
    pos: Vec2D,
    next: Vec<(usize, f64)>,
}

impl MarkovState {
    pub fn new(out: &str, next: Vec<(usize, f64)>) -> MarkovState {
        MarkovState {
            output: out.to_owned(),
            color: Rgba::random(),
            pos: Vec2D::default(),
            next,
        }
    }
}

pub struct MarkovChain {
    states: Vec<MarkovState>,
    active: usize,
    active_color: Rgba,
    ticks: f64,
    output: String,
}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            states: vec![],
            active: 0,
            active_color: Rgba::random(),
            ticks: 0.0,
            output: "".to_owned(),
        }
    }
}

impl Drawable for MarkovChain {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for (ind, state) in self.states.iter().enumerate() {
            let color = if ind == self.active {
                self.active_color
            } else {
                state.color
            };

            for (next, prob) in state.next.iter() {
                let next_pos = self.states[*next].pos;
                let between = next_pos - state.pos;
                let mut tan = between.tangent();
                tan.set_abs(between.abs() / 8.0);
                let mid_point = state.pos + 0.5 * between + tan;

                let line_size = *prob * LINE_SIZE;

                line(
                    state.color.into(),
                    line_size,
                    [state.pos.x, state.pos.y, mid_point.x, mid_point.y],
                    transform,
                    gl,
                );
                line(
                    state.color.into(),
                    line_size,
                    [mid_point.x, mid_point.y, next_pos.x, next_pos.y],
                    transform,
                    gl,
                );
            }
            ellipse(
                color.into(),
                [
                    state.pos.x - STATE_SIZE / 2.0,
                    state.pos.y - STATE_SIZE / 2.0,
                    STATE_SIZE,
                    STATE_SIZE,
                ],
                transform,
                gl,
            );
            Text::new(18)
                .draw_pos(
                    &state.output,
                    [state.pos.x, state.pos.y],
                    ctx.glyphs,
                    &ctx.context.draw_state,
                    transform,
                    gl,
                )
                .unwrap();
        }
    }
}

impl Updatable for MarkovChain {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.ticks += ctx.args.dt;
        if self.ticks < UPDATE_TIME {
            return;
        }
        self.ticks = 0.0;

        let next_possible = &self.states[self.active].next;
        let next_choice = rand::random::<f64>();

        let mut next_ind = 0;
        while next_possible[next_ind].1 < next_choice {
            next_ind += 1;
            if next_ind + 1 == next_possible.len() {
                break;
            }
        }

        self.active = next_possible[next_ind].0;
        self.output += " ";
        self.output += &self.states[self.active].output;
        println!("{}", self.output);
    }
}

impl EventHandler for MarkovChain {}

impl Runnable for MarkovChain {
    fn setup(&mut self, ctx: &mut SetupContext) {
        let content = read_to_string(INPUT_FILE).unwrap();
        let mut next = HashMap::new();
        for line in content.lines() {
            let words: Vec<&str> = line.split(" ").collect();
            for pair in words.windows(2) {
                let word = pair[0].to_lowercase();
                match next.get_mut(&word) {
                    None => {
                        next.insert(word, vec![pair[1].to_lowercase()]);
                    }
                    Some(words) => words.push(pair[1].to_lowercase()),
                };
            }
            next.insert(
                words.last().unwrap().to_lowercase(),
                vec![words.first().unwrap().to_lowercase()],
            );
        }

        for (word, next_words) in next.iter() {
            let total_next = next_words.len() as f64;
            let mut next_probs = vec![];
            for next_word in next_words {
                let (next_ind, _) = next
                    .iter()
                    .enumerate()
                    .find(|(_, (w, _))| *w == next_word)
                    .unwrap();
                match next_probs.iter_mut().find(|(ind, _)| *ind == next_ind) {
                    None => next_probs.push((next_ind, 1.0 / total_next)),
                    Some((_, prob)) => *prob += 1.0 / total_next,
                };
            }

            let mut next_state = MarkovState::new(word, next_probs);
            next_state.pos = Vec2D::new(
                rand_between(STATE_SIZE, ctx.window_width - STATE_SIZE),
                rand_between(STATE_SIZE, ctx.window_height - STATE_SIZE),
            );
            self.states.push(next_state);
        }
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Markov Chain".to_owned(),
        }
    }
}
