use graphics::{
    colors::rgba, line, polygon, ui_elements::TextField, Drawable, DrawingContext, EventHandler,
    Graphics, InputContext, Runnable, SetupContext, Updatable, UpdateContext, WindowConfig,
};
use piston::{Button, ButtonState, Key};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const FIELD_H: f64 = 200.0;
const FIELD_W: f64 = 300.0;

const REPLACEMENT_TIME: f64 = 1.0;

const PAUSE_X: f64 = 10.0;
const PAUSE_W: f64 = 3.0;
const PAUSE_Y: f64 = 10.0;
const PAUSE_H: f64 = 20.0;

pub struct WordInteractor {
    text_field: TextField,
    replacement: TextField,
    delta_t: f64,
    on: bool,
}

impl WordInteractor {
    pub fn new() -> WordInteractor {
        WordInteractor {
            text_field: TextField::new(0.0, 0.0, FIELD_W, FIELD_H, 18),
            replacement: TextField::new(0.0, 0.0, FIELD_W, FIELD_H / 10.0, 18),
            delta_t: 0.0,
            on: false,
        }
    }
}

impl Drawable for WordInteractor {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        self.text_field.draw(ctx, gl);
        self.replacement.draw(ctx, gl);

        let transform = ctx.id_trans();
        if self.on {
            polygon(
                rgba::WHITE.into(),
                &[
                    [PAUSE_X, PAUSE_Y],
                    [PAUSE_X, PAUSE_Y + PAUSE_H],
                    [PAUSE_X + 4.0 * PAUSE_W, PAUSE_Y + PAUSE_H / 2.0],
                ],
                transform,
                gl,
            );
        } else {
            line(
                rgba::WHITE.into(),
                PAUSE_W,
                [PAUSE_X, PAUSE_Y, PAUSE_X, PAUSE_Y + PAUSE_H],
                transform,
                gl,
            );
            line(
                rgba::WHITE.into(),
                PAUSE_W,
                [
                    PAUSE_X + 4.0 * PAUSE_W,
                    PAUSE_Y,
                    PAUSE_X + 4.0 * PAUSE_W,
                    PAUSE_Y + PAUSE_H,
                ],
                transform,
                gl,
            );
        }
    }
}

impl Updatable for WordInteractor {
    fn update(&mut self, ctx: &mut UpdateContext) {
        if !self.on {
            return;
        }

        self.delta_t += ctx.args.dt;
        if self.delta_t < REPLACEMENT_TIME {
            return;
        }
        self.delta_t = 0.0;

        let repl_target = self.replacement.text();
        let repl_text = self.text_field.text();
        let mut repl_words: Vec<&str> = repl_text.split(" ").collect();
        let rand_ind = rand::random::<usize>() % repl_words.len();
        repl_words[rand_ind] = &repl_target;
        let replaced = repl_words.join(" ");
        self.text_field.set_text(replaced);
    }
}

impl EventHandler for WordInteractor {
    fn handle_input(&mut self, ctx: &InputContext) {
        self.text_field.handle_input(ctx);
        self.replacement.handle_input(ctx);

        if !self.text_field.active()
            && !self.replacement.active()
            && ctx.args.state == ButtonState::Release
            && ctx.args.button == Button::Keyboard(Key::Space)
        {
            self.on = !self.on;
        }
    }
}

impl Runnable for WordInteractor {
    fn setup(&mut self, ctx: &mut SetupContext) {
        let size = self.text_field.size();
        self.text_field.set_pos(
            (ctx.window_width - size[0]) / 2.0,
            (ctx.window_height - size[1]) / 2.0,
        );

        let pos = self.text_field.pos();
        self.replacement.set_pos(pos.x, pos.y - FIELD_H + 100.0);
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Word Interactor".to_owned(),
        }
    }
}
