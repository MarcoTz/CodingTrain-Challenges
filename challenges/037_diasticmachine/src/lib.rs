use graphics::{
    colors::Rgba, Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use piston_window::text::Text;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const FONT_SIZE: u32 = 18;

pub struct Diastic {
    input_word: String,
    word_color: Rgba,
    input_text: String,
    next_char: usize,
    input_pos: usize,
    output: String,
}

impl Diastic {
    pub fn new() -> Diastic {
        Diastic {
            input_word: Self::load_word(),
            word_color: Rgba::random(),
            input_text: Self::load_text(),
            next_char: 0,
            input_pos: 0,
            output: "".to_owned(),
        }
    }

    fn load_word() -> String {
        "testing".to_owned()
    }

    fn load_text() -> String {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n
            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n
            Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n
            Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_owned()
    }
}

impl Drawable for Diastic {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.center_trans();
        Text::new_color(self.word_color.into(), FONT_SIZE)
            .draw_pos(
                &self.input_word,
                [0.0, -(FONT_SIZE as f64)],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
        Text::new_color(self.word_color.into(), FONT_SIZE)
            .draw_pos(
                &self.output,
                [
                    -(FONT_SIZE as f64 * self.output.len() as f64) / 4.0,
                    FONT_SIZE as f64,
                ],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
    }
}

impl Updatable for Diastic {
    fn update(&mut self, _: &mut UpdateContext) {
        let next_char = if let Some(ch) = self.input_word.chars().nth(self.next_char) {
            ch.to_ascii_lowercase()
        } else {
            return;
        };

        let mut next_word = "";
        for (ind, word) in self.input_text.split(" ").skip(self.input_pos).enumerate() {
            if let Some(ch) = word.chars().nth(self.next_char) {
                if ch.to_ascii_lowercase() == next_char {
                    next_word = word;
                    self.input_pos += ind + 1;
                    break;
                }
            }
        }

        if next_word.is_empty() {
            next_word = "____";
        }
        self.next_char += 1;
        self.output += " ";
        self.output += next_word;
    }
}

impl EventHandler for Diastic {}

impl Runnable for Diastic {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Diastic".to_owned(),
        }
    }
}
