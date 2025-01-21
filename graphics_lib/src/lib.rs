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

pub struct HandlerContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub args: &'a ButtonArgs,
}

impl<'a> DrawingContext<'a> {
    pub fn id_trans(&self) -> [[f64; 3]; 2] {
        self.context.transform.scale(1.0, 1.0)
    }
    pub fn center_trans(&self) -> [[f64; 3]; 2] {
        self.context.transform.trans(
            self.args.window_size[0] / 2.0,
            self.args.window_size[1] / 2.0,
        )
    }
}

pub trait Runnable: Drawable + Updatable + InputHandler {
    fn window_size(&self) -> Size;
}

pub trait InputHandler {
    fn handle(&mut self, _: &HandlerContext) {}
}

pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics);
}

pub trait Updatable {
    fn update(&mut self, _: &UpdateContext) {}
}
