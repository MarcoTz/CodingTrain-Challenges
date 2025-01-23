use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct DrawingContext<'a> {
    pub context: Context,
    pub args: &'a RenderArgs,
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
pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics);
}
