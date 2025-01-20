pub mod app;
pub mod point;

use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use piston::ButtonArgs;
use window::Size;

pub struct DrawingContext<'a> {
    pub context: Context,
    pub args: &'a RenderArgs,
}

pub struct UpdateContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub args: &'a UpdateArgs,
}

impl<'a> DrawingContext<'a> {
    pub fn center_trans(&self) -> [[f64; 3]; 2] {
        self.context.transform.trans(
            self.args.window_size[0] / 2.0,
            self.args.window_size[1] / 2.0,
        )
    }
}

pub trait Runnable {
    fn window_size(&self) -> Size;

    fn to_draw(&self) -> Vec<&dyn Drawable> {
        vec![]
    }
    fn to_update(&mut self) -> Vec<&mut dyn Updatable> {
        vec![]
    }
    fn handlers(&mut self) -> Vec<&mut dyn InputHandler> {
        vec![]
    }
}

pub trait InputHandler {
    fn handle(&mut self, _: &ButtonArgs) {}
}

pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics);
}

pub trait Updatable {
    fn update(&mut self, ctx: &UpdateContext);
}
