use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

pub trait Drawable {
    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics);
    fn update(&mut self, args: &UpdateArgs);
}
