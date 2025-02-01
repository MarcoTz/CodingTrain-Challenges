use graphics::{rectangle, types::Color /*DrawState*/};
use graphics_lib::{
    vec2d::Vec2D, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    SetupContext, Updatable, UpdateContext, WindowConfig,
};
use piston::{Button, ButtonState, Key, ResizeArgs};
//use piston_window::{text, Glyphs, TextureSettings};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const COLOR_INSIDE: Color = [1.0, 0.0, 0.0, 1.0];
const TOLERANCE: f64 = 0.01;
const RESOLUTION: f64 = 2.0;

pub struct SuperShape {
    n_1: f64,
    n_2: f64,
    n_3: f64,
    m: f64,
    a: f64,
    b: f64,
    computed: Vec<Vec<bool>>,
    paused: bool,
}

impl SuperShape {
    pub fn new() -> SuperShape {
        SuperShape {
            m: 1.0,
            n_1: 0.3,
            n_2: 0.3,
            n_3: 0.3,
            a: 1.0,
            b: 1.0,
            computed: vec![],
            paused: true,
        }
    }

    fn inside(&self, pt: Vec2D) -> bool {
        let arg = pt.arg();
        let r = pt.abs();
        let arg_cos = (arg * self.m / 4.0).cos() / self.a;
        let arg_sin = (arg * self.m / 4.0).sin() / self.b;
        let sum = arg_cos.abs().powf(self.n_2) + arg_sin.abs().powf(self.n_3);
        let res = 1.0 / sum.powf(1.0 / self.n_1);
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
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();

        for x in 0..(ctx.args.window_size[0].ceil() as usize) {
            for y in 0..(ctx.args.window_size[1].ceil() as usize) {
                if !self.computed[x][y] {
                    continue;
                }
                rectangle(COLOR_INSIDE, [x as f64, y as f64, 5.0, 5.0], transform, gl);
            }
        }
        /*let mut glyphs = todo!(); Glyphs::from_bytes(
            include_bytes!("font.ttf",),
            &ctx.texture_context,
            TextureSettings::new(),
        )
        .unwrap()*/
        /*text::Text::new_color([0.0, 0.0, 1.0, 1.0], 18).draw(
            &format!("{}", self.m),
            &mut glyphs,
            &DrawState::new_alpha(),
            transform,
            gl,
        );*/
    }
}

impl Updatable for SuperShape {
    fn update(&mut self, ctx: &UpdateContext) {
        if !self.paused {
            self.paused = true;
            self.m += 1.0;
            self.compute(ctx.window_width, ctx.window_height)
        }
    }
}

impl EventHandler for SuperShape {
    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.compute(ctx.window_size[0], ctx.window_size[1]);
    }

    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.button == Button::Keyboard(Key::Space) && ctx.args.state == ButtonState::Release
        {
            self.paused = false;
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
