use graphics::{rectangle, types::Color};
use graphics_lib::{
    vec2d::Vec2D, Drawable, DrawingContext, EventHandler, Runnable, Updatable, UpdateContext,
    WindowConfig,
};
use opengl_graphics::GlGraphics;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const X_RES: f64 = 100.0;
const Y_RES: f64 = 100.0;

const COLOR_INSIDE: Color = [1.0, 0.0, 0.0, 1.0];
const COLOR_OUTSIDE: Color = [0.0, 1.0, 0.0, 1.0];

pub struct SuperShape {
    center: Vec2D,
    n_1: f64,
    n_2: f64,
    n_3: f64,
    m: f64,
    a: f64,
    b: f64,
}

impl SuperShape {
    pub fn new() -> SuperShape {
        SuperShape {
            center: Vec2D::default(),
            m: 0.0,
            n_1: 0.0,
            n_2: 0.0,
            n_3: 0.0,
            a: 2.0,
            b: 2.0,
        }
    }

    fn inside(&self, pt: Vec2D) -> bool {
        let arg = pt.arg();
        let r = pt.abs();
        let arg_cos = (arg * self.m / 4.0).cos() / self.a;
        let arg_sin = (arg * self.m / 4.0).sin() / self.b;
        let sum = arg_cos.abs().powf(self.n_2) + arg * arg_sin.abs().powf(self.n_3);
        sum.powf(self.n_1) == 1.0 / r
    }
}

impl Drawable for SuperShape {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.center_trans();
        let x_min = -(ctx.args.window_size[0] / 2.0).floor() as i64;
        let x_max = (ctx.args.window_size[0] / 2.0).floor() as i64;
        let y_min = -(ctx.args.window_size[0] / 2.0).floor() as i64;
        let y_max = (ctx.args.window_size[1] / 2.0).floor() as i64;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let color = if self.inside(Vec2D::new(x as f64, y as f64)) {
                    COLOR_INSIDE
                } else {
                    COLOR_OUTSIDE
                };
                rectangle(color, [x as f64, y as f64, 3.0, 3.0], transform, gl);
            }
        }
    }
}

impl Updatable for SuperShape {
    fn update(&mut self, ctx: &UpdateContext) {}
}

impl EventHandler for SuperShape {}

impl Runnable for SuperShape {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Supershape".to_owned(),
        }
    }
}
