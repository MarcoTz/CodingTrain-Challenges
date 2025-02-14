use graphics::{
    colors::Rgba, Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext,
    Updatable, UpdateContext, WindowConfig,
};
use piston_window::text::Text;
use std::{collections::HashMap, fs::read_to_string};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;
const FONT_SIZE: u32 = 18;

const AFINN_FILE: &str = "./challenges/044_afinn/afinn.txt";
const INPUT: &str = "./challenges/044_afinn/input.txt";

pub struct Afinn {
    sentiments: HashMap<String, i64>,
    input: Vec<String>,
    next_word: usize,
    afinn: i64,
    text_color: Rgba,
}

impl Afinn {
    pub fn new() -> Afinn {
        Afinn {
            sentiments: HashMap::new(),
            input: vec![],
            next_word: 0,
            afinn: 0,
            text_color: Rgba::random(),
        }
    }
}

impl Drawable for Afinn {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let avg_score = self.afinn as f64 / (self.next_word as f64 - 1.0);
        Text::new_color(self.text_color.into(), FONT_SIZE)
            .draw_pos(
                &avg_score.to_string(),
                [ctx.args.window_size[0] / 2.0, ctx.args.window_size[1] / 2.0],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
        println!("afinn:{}", self.afinn);
    }
}

impl Updatable for Afinn {
    fn update(&mut self, _: &mut UpdateContext) {
        if self.next_word >= self.input.len() {
            return;
        }

        let next = &self.input[self.next_word];
        if !self.sentiments.contains_key(next) {
            println!("{next} not found");
            self.next_word += 1;
            return;
        }
        let sent = *self.sentiments.get(next).unwrap();
        self.afinn += sent;
        self.next_word += 1;
    }
}

impl EventHandler for Afinn {}

impl Runnable for Afinn {
    fn setup(&mut self, _: &mut SetupContext) {
        let contents = read_to_string(AFINN_FILE).unwrap();
        for line in contents.lines() {
            let mut parts = line.split('\t');
            let word = parts.next().unwrap();
            let val = parts.next().unwrap().parse::<i64>().unwrap();
            self.sentiments.insert(word.to_owned(), val);
        }

        let contents = read_to_string(INPUT)
            .unwrap()
            .split(" ")
            .map(|s| {
                s.replace("\"", "")
                    .replace("'", "")
                    .replace(",", "")
                    .replace(".", "")
                    .to_lowercase()
            })
            .collect();
        self.input = contents;
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Afinn".to_owned(),
        }
    }
}
