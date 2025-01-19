use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

pub type TransformMatrix = [[f64; 3]; 2];
pub trait Drawable {
    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics, transform: TransformMatrix);
    fn update(&mut self, args: &UpdateArgs);
}
