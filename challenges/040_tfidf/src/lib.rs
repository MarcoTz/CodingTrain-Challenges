use graphics::{
    Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const TEXT_FILE: &str = "challenges/040_tfidf/text.txt";
const FONT_SIZE: u32 = 18;
const SIZE_MULT: f64 = 2000.0;

const FRICTION: f64 = 0.99;

mod word_freq;
use word_freq::WordFreq;

pub struct TfIdf {
    word_counts: Vec<WordFreq>,
}

impl TfIdf {
    pub fn new() -> TfIdf {
        TfIdf {
            word_counts: vec![],
        }
    }

    fn calc_tf(&mut self) {
        let count_sums = self.word_counts.iter().map(|freq| freq.cnt).sum::<u64>() as f64;
        for freq in self.word_counts.iter_mut() {
            freq.tf = freq.cnt as f64 / (count_sums - freq.cnt as f64);
        }
    }
}

impl Drawable for TfIdf {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for freq in self.word_counts.iter() {
            freq.draw(ctx, gl);
        }
    }
}

impl Updatable for TfIdf {
    fn update(&mut self, ctx: &mut UpdateContext) {
        let center = Vec2D::new(ctx.window_width / 2.0, ctx.window_height / 2.0);
        for i in 0..self.word_counts.len() {
            for j in (i + 1)..self.word_counts.len() {
                if self.word_counts[i].collides(&self.word_counts[j]) {
                    let pos_i = self.word_counts[i].pos;
                    let r_i = self.word_counts[i].radius();
                    let pos_j = self.word_counts[j].pos;
                    let r_j = self.word_counts[j].radius();
                    let mut diff = pos_i - pos_j;
                    diff.set_abs(diff.abs() - r_i - r_j);
                    self.word_counts[i].pos -= diff;
                    self.word_counts[j].pos += diff;

                    let tangent = diff.tangent();
                    let inv_det = tangent.x * diff.y - diff.x * tangent.y;

                    let vel_i = self.word_counts[i].vel;
                    let vel_i_abs = vel_i.abs();
                    let vel_j = self.word_counts[j].vel;
                    let vel_j_abs = vel_j.abs();

                    let other_basis_y_i = vel_i.x * tangent.y + vel_j.y * diff.y;
                    let other_basis_y_j = vel_j.x * tangent.y + vel_j.y * diff.y;
                    let new_vel_i = Vec2D::new(
                        -diff.x * other_basis_y_i / inv_det,
                        tangent.x * other_basis_y_i / inv_det,
                    );
                    let new_vel_j = Vec2D::new(
                        -diff.x * other_basis_y_j / inv_det,
                        tangent.x * other_basis_y_i / inv_det,
                    );
                    self.word_counts[i].vel = new_vel_i;
                    self.word_counts[i].vel.set_abs(vel_i_abs);
                    self.word_counts[j].vel = new_vel_j;
                    self.word_counts[j].vel.set_abs(vel_j_abs);
                }
            }
            self.word_counts[i].update(ctx);
            self.word_counts[i].acc = center - self.word_counts[i].pos;
        }
    }
}

impl EventHandler for TfIdf {}

impl Runnable for TfIdf {
    fn setup(&mut self, ctx: &mut SetupContext) {
        let contents = std::fs::read_to_string(TEXT_FILE).unwrap();
        for line in contents.lines() {
            for word in line.split(" ") {
                let word = word.to_lowercase();
                if word.is_empty() {
                    continue;
                }
                match self.word_counts.iter_mut().find(|freq| freq.word == word) {
                    None => {
                        self.word_counts.push(WordFreq::new(
                            word,
                            rand_between(0.0, ctx.window_width),
                            rand_between(0.0, ctx.window_height),
                        ));
                    }
                    Some(freq) => freq.cnt += 1,
                }
            }
        }

        self.calc_tf();
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "TfIdf".to_owned(),
        }
    }
}
