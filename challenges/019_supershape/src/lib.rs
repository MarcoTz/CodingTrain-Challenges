use graphics::{
    rectangle, ui_elements::Button, Color, Drawable, DrawingContext, EventHandler, Graphics,
    InputContext, Runnable, SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use piston::{Button as PisButton, ButtonState, Key, ResizeArgs};
use std::fmt;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const COLOR_INSIDE: Color = [1.0, 0.0, 0.0, 1.0];
const TOLERANCE: f64 = 0.01;
const RESOLUTION: f64 = 2.0;

const BUTTON_COLOR: Color = [0.0, 0.4, 0.8, 1.0];
const TEXT_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const FONT_SIZE: u32 = 18;
const BUTTON_WIDTH: f64 = 60.0;
const BUTTON_DIST: f64 = 5.0;
const BUTTON_HEIGHT: f64 = 30.0;
const VAL_CHANGE: f64 = 0.1;

#[derive(Clone, Copy)]
enum ConstLabel {
    M,
    N1,
    N2,
    N3,
    A,
    B,
}

impl Default for ConstLabel {
    fn default() -> ConstLabel {
        ConstLabel::M
    }
}

impl ConstLabel {
    fn prev(self) -> ConstLabel {
        match self {
            ConstLabel::B => ConstLabel::A,
            ConstLabel::A => ConstLabel::N3,
            ConstLabel::N3 => ConstLabel::N2,
            ConstLabel::N2 => ConstLabel::N1,
            ConstLabel::N1 => ConstLabel::M,
            ConstLabel::M => ConstLabel::B,
        }
    }

    fn next(self) -> Option<ConstLabel> {
        match self {
            ConstLabel::M => Some(ConstLabel::N1),
            ConstLabel::N1 => Some(ConstLabel::N2),
            ConstLabel::N2 => Some(ConstLabel::N3),
            ConstLabel::N3 => Some(ConstLabel::A),
            ConstLabel::A => Some(ConstLabel::B),
            ConstLabel::B => None,
        }
    }

    fn default_value(&self) -> f64 {
        match self {
            ConstLabel::M => 1.0,
            ConstLabel::N1 => 0.3,
            ConstLabel::N2 => 0.3,
            ConstLabel::N3 => 0.3,
            ConstLabel::A => 1.0,
            ConstLabel::B => 1.0,
        }
    }

    fn start_x(self) -> f64 {
        if matches!(self, ConstLabel::M) {
            return BUTTON_DIST;
        }
        self.prev().start_x() + 2.0 * (BUTTON_DIST + BUTTON_WIDTH)
    }
}

impl fmt::Display for ConstLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstLabel::M => f.write_str("m"),
            ConstLabel::N1 => f.write_str("n1"),
            ConstLabel::N2 => f.write_str("n2"),
            ConstLabel::N3 => f.write_str("n3"),
            ConstLabel::A => f.write_str("a"),
            ConstLabel::B => f.write_str("b"),
        }
    }
}

struct ShapeConstant {
    label: ConstLabel,
    value: f64,
    inc_button: Button,
    dec_button: Button,
}

impl ShapeConstant {
    pub fn new(label: ConstLabel, val: f64) -> ShapeConstant {
        let inc_x = label.start_x();
        let dec_x = inc_x + BUTTON_WIDTH + BUTTON_DIST;
        let y = HEIGHT - BUTTON_HEIGHT;
        let inc_label = format!("{label}+");
        let dec_label = format!("{label}-");
        ShapeConstant {
            label,
            value: val,
            inc_button: Button::new(
                inc_x,
                y,
                BUTTON_WIDTH,
                BUTTON_HEIGHT,
                BUTTON_COLOR,
                &inc_label,
                TEXT_COLOR,
                FONT_SIZE,
            ),
            dec_button: Button::new(
                dec_x,
                y,
                BUTTON_WIDTH,
                BUTTON_HEIGHT,
                BUTTON_COLOR,
                &dec_label,
                TEXT_COLOR,
                FONT_SIZE,
            ),
        }
    }
}

impl Drawable for ShapeConstant {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        self.inc_button.draw(ctx, gl);
        self.dec_button.draw(ctx, gl);
    }
}

impl EventHandler for ShapeConstant {
    fn handle_input(&mut self, ctx: &InputContext) {
        if self.inc_button.clicked(ctx.mouse_pos, ctx.args) {
            self.value += VAL_CHANGE;
        }
        if self.dec_button.clicked(ctx.mouse_pos, ctx.args) {
            self.value -= VAL_CHANGE;
        }
    }
}

pub struct SuperShape {
    constants: Vec<ShapeConstant>,
    computed: Vec<Vec<bool>>,
    paused: bool,
}

impl SuperShape {
    pub fn new() -> SuperShape {
        let mut new = SuperShape {
            constants: vec![],
            computed: vec![],
            paused: true,
        };
        new.create_constants();
        new
    }

    fn create_constants(&mut self) {
        self.constants.clear();
        let mut next_label = Some(ConstLabel::default());
        while next_label.is_some() {
            let label = next_label.unwrap();
            self.constants
                .push(ShapeConstant::new(label, label.default_value()));
            next_label = label.next();
        }
    }

    fn inside(&self, pt: Vec2D) -> bool {
        let arg = pt.arg();
        let r = pt.abs();
        let arg_cos = (arg * self.constants[0].value / 4.0).cos() / self.constants[4].value;
        let arg_sin = (arg * self.constants[0].value / 4.0).sin() / self.constants[5].value;
        let sum = arg_cos.abs().powf(self.constants[2].value)
            + arg_sin.abs().powf(self.constants[3].value);
        let res = 1.0 / sum.powf(1.0 / self.constants[1].value);
        res >= r - TOLERANCE && res <= r + TOLERANCE
    }

    fn compute(&mut self, window_width: f64, window_height: f64) {
        let start_x = -RESOLUTION;
        let start_y = RESOLUTION;
        let step_x = (2.0 * RESOLUTION) / (window_width);
        let step_y = (2.0 * RESOLUTION) / (window_height);

        self.computed.clear();

        for i in 0..=window_width.ceil() as usize {
            self.computed.push(vec![]);
            for j in 0..=window_height.ceil() as usize {
                let x = start_x + i as f64 * step_x;
                let y = start_y - j as f64 * step_y;
                let inside = self.inside(Vec2D {
                    x: x as f64,
                    y: y as f64,
                });
                self.computed[i].push(inside);
            }
        }
    }
}

impl Drawable for SuperShape {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();

        for x in 0..(ctx.args.window_size[0].ceil() as usize) {
            for y in 0..(ctx.args.window_size[1].ceil() as usize) {
                if !self.computed[x][y] {
                    continue;
                }
                rectangle(COLOR_INSIDE, [x as f64, y as f64, 5.0, 5.0], transform, gl);
            }
        }

        for cons in self.constants.iter() {
            cons.draw(ctx, gl);
        }
    }
}

impl Updatable for SuperShape {
    fn update(&mut self, ctx: &UpdateContext) {
        self.compute(ctx.window_width, ctx.window_height);
    }
}

impl EventHandler for SuperShape {
    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.create_constants();
        self.compute(ctx.window_size[0], ctx.window_size[1]);
    }

    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.button == PisButton::Keyboard(Key::Space)
            && ctx.args.state == ButtonState::Release
        {
            self.paused = false;
        }

        for cons in self.constants.iter_mut() {
            cons.handle_input(ctx);
        }
    }
}

impl Runnable for SuperShape {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Supershape".to_owned(),
        }
    }

    fn setup(&mut self, ctx: &SetupContext) {
        self.compute(ctx.window_height, ctx.window_width);
    }
}
