use graphics::{
    colors::Rgba, ui_elements::TextField, Drawable, DrawingContext, EventHandler, Graphics,
    InputContext, Runnable, SetupContext, Updatable, UpdateContext, WindowConfig,
};
use piston::{Button, ButtonState, Key};
use piston_window::text::Text;
use std::{convert::Infallible, fmt, fs::read_to_string, path::PathBuf, str::FromStr};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;
const FONT_SIZE: u32 = 18;
const SENTENCE_LIST: &str = "challenges/039_madlibs/sentences.txt";
const INPUT_W: f64 = 200.0;

#[derive(Debug)]
enum HoleType {
    Adjective,
    Noun,
    Verb,
    Adverb,
    Any,
}

impl FromStr for HoleType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<HoleType, Self::Err> {
        let s = s.to_lowercase();
        match s {
            s if s.contains("adjective") => Ok(HoleType::Adjective),
            s if s.contains("noun") => Ok(HoleType::Noun),
            s if s.contains("adverb") => Ok(HoleType::Adverb),
            s if s.contains("verb") => Ok(HoleType::Verb),
            _ => Ok(HoleType::Any),
        }
    }
}

impl fmt::Display for HoleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HoleType::Adjective => f.write_str("(Adjective)"),
            HoleType::Noun => f.write_str("(Noun)"),
            HoleType::Adverb => f.write_str("(Adverb)"),
            HoleType::Verb => f.write_str("(Verb)"),
            HoleType::Any => f.write_str("(any)"),
        }
    }
}

#[derive(Debug)]
enum WordOrHole {
    Word(String),
    Hole(HoleType),
}

impl FromStr for WordOrHole {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<WordOrHole, Self::Err> {
        if s.starts_with('(') {
            Ok(WordOrHole::Hole(HoleType::from_str(s).unwrap()))
        } else {
            Ok(WordOrHole::Word(s.to_owned()))
        }
    }
}

impl fmt::Display for WordOrHole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WordOrHole::Word(word) => f.write_str(word),
            WordOrHole::Hole(ty) => ty.fmt(f),
        }
    }
}

struct MadSentence {
    sentence: Vec<WordOrHole>,
    solution: Vec<String>,
}

impl MadSentence {
    fn len(&self) -> usize {
        self.sentence.len()
    }

    fn num_holes(&self) -> usize {
        self.sentence
            .iter()
            .filter(|wh| matches!(wh, WordOrHole::Hole(_)))
            .count()
    }
}

impl fmt::Display for MadSentence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sol_ind = 0;
        for wh in self.sentence.iter() {
            match wh {
                WordOrHole::Word(word) => {
                    write!(f, "{word}")?;
                }
                WordOrHole::Hole(hole) => match self.solution.get(sol_ind) {
                    None => {
                        write!(f, "{hole}")?;
                    }
                    Some(sol) => {
                        write!(f, "{sol}")?;
                        sol_ind += 1;
                    }
                },
            }
            write!(f, " ")?;
        }
        Ok(())
    }
}

impl FromStr for MadSentence {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<MadSentence, Self::Err> {
        Ok(MadSentence {
            sentence: s
                .split(" ")
                .map(|word| WordOrHole::from_str(word).unwrap())
                .collect(),
            solution: vec![],
        })
    }
}

pub struct MadLibs {
    sentences: Vec<MadSentence>,
    current_sentence: usize,
    text_color: Rgba,
    input_field: TextField,
}

impl MadLibs {
    pub fn new() -> MadLibs {
        MadLibs {
            sentences: vec![],
            current_sentence: 0,
            text_color: Rgba::random(),
            input_field: TextField::new(0.0, 0.0, INPUT_W, FONT_SIZE as f64 + 2.0, FONT_SIZE),
        }
    }

    fn load_sentences(&mut self, path: PathBuf) {
        let contents = read_to_string(path).unwrap();
        for sentence in contents.lines() {
            let sentence = MadSentence::from_str(sentence).unwrap();
            self.sentences.push(sentence);
        }
    }
}

impl Drawable for MadLibs {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let current_sentence = &self.sentences[self.current_sentence];
        let sentence_str = current_sentence.to_string();

        let mut middle = sentence_str.len() / 2;
        while sentence_str.chars().nth(middle) != Some(' ') {
            middle += 1;
            if middle >= sentence_str.len() {
                middle = sentence_str.len() / 2;
            }
        }
        let (fst_str, snd_str) = sentence_str.split_at(middle);

        let text = Text::new_color(self.text_color.into(), FONT_SIZE);
        let start_y = ctx.args.window_size[1] / 3.0;
        text.draw_pos(
            &fst_str,
            [
                ctx.args.window_size[0] / 2.0 - current_sentence.len() as f64 * FONT_SIZE as f64,
                start_y,
            ],
            ctx.glyphs,
            &ctx.context.draw_state,
            transform,
            gl,
        )
        .unwrap();
        text.draw_pos(
            &snd_str,
            [
                ctx.args.window_size[0] / 2.0 - current_sentence.len() as f64 * FONT_SIZE as f64,
                start_y + 2.0 + FONT_SIZE as f64,
            ],
            ctx.glyphs,
            &ctx.context.draw_state,
            transform,
            gl,
        )
        .unwrap();

        self.input_field.draw(ctx, gl);
    }
}

impl Updatable for MadLibs {
    fn update(&mut self, _: &mut UpdateContext) {}
}

impl EventHandler for MadLibs {
    fn handle_input(&mut self, ctx: &InputContext) {
        self.input_field.handle_input(ctx);
        if ctx.args.state != ButtonState::Release
            || ctx.args.button != Button::Keyboard(Key::Return)
        {
            return;
        }

        let current_sentence = &self.sentences[self.current_sentence];
        if current_sentence.solution.len() == current_sentence.num_holes() {
            self.current_sentence += 1;
            self.current_sentence = self.current_sentence % self.sentences.len();
            self.sentences[self.current_sentence].solution.clear();
            return;
        }

        let current_word = self.input_field.text();
        self.sentences[self.current_sentence]
            .solution
            .push(current_word);
        self.input_field.clear();
    }
}

impl Runnable for MadLibs {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.load_sentences(PathBuf::from(SENTENCE_LIST));
        let size = self.input_field.size();
        self.input_field.set_pos(
            (ctx.window_width - size[0]) / 2.0,
            2.0 * ctx.window_height / 3.0,
        );
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "MadLibs".to_owned(),
        }
    }
}
