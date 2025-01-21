pub mod app;
pub mod vec2d;

use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use piston::{ButtonArgs, ResizeArgs};
use window::Size;

pub fn rand_between(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max - min)
}

pub struct DrawingContext<'a> {
    pub context: Context,
    pub args: &'a RenderArgs,
}

pub struct UpdateContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub args: &'a UpdateArgs,
}

pub struct InputContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub mouse_pos: [f64; 2],
    pub args: &'a ButtonArgs,
}

pub struct SetupContext {
    pub window_height: f64,
    pub window_width: f64,
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

pub trait Runnable: Drawable + Updatable + EventHandler {
    fn window_size(&self) -> Size;
    fn setup(&mut self, _: &SetupContext) {}
}

pub trait EventHandler {
    fn handle_input(&mut self, _: &InputContext) {}
    fn handle_resize(&mut self, _: &ResizeArgs) {}
}

pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics);
}

pub trait Updatable {
    fn update(&mut self, _: &UpdateContext) {}
}
