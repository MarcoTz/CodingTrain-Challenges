use crate::Graphics;
use gfx_device_gl::Factory;
use graphics::{Context, Transformed};
use piston::RenderArgs;
use piston_window::{G2dTextureContext, TextureContext};

pub struct DrawingContext<'a> {
    pub context: Context,
    pub args: &'a RenderArgs,
    pub factory: Factory,
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

    pub fn create_texture_context(&self) -> G2dTextureContext {
        let mut factory = self.factory.clone();
        let encoder = factory.create_command_buffer().into();
        TextureContext { factory, encoder }
    }
}

pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics);
}
