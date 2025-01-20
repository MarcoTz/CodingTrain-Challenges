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

pub trait Runnable {
    fn window_size(&self) -> Size;

    type DrawingArgs;
    type UpdateArgs;
    type HandlerArgs;

    fn to_draw(
        &self,
    ) -> Vec<(
        &dyn Drawable<DrawingArgs = Self::DrawingArgs>,
        &Self::DrawingArgs,
    )> {
        vec![]
    }
    fn to_update(
        &mut self,
    ) -> Vec<(
        &mut dyn Updatable<UpdateArgs = Self::UpdateArgs>,
        &Self::UpdateArgs,
    )> {
        vec![]
    }
    fn handlers(
        &mut self,
    ) -> Vec<(
        &mut dyn InputHandler<HandlerArgs = Self::HandlerArgs>,
        &Self::HandlerArgs,
    )> {
        vec![]
    }
}

pub trait InputHandler {
    type HandlerArgs;
    fn handle(&mut self, bt_args: &ButtonArgs, args: &Self::HandlerArgs);
}

pub trait Drawable {
    type DrawingArgs;
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics, args: &Self::DrawingArgs);
}

pub trait Updatable {
    type UpdateArgs;
    fn update(&mut self, ctx: &UpdateContext, args: &Self::UpdateArgs);
}
