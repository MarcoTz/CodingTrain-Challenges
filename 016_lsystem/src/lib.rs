use graphics_lib::{Drawable, DrawingContext, EventHandler, Runnable, Updatable, UpdateContext};
use opengl_graphics::GlGraphics;
use piston::Size;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

pub struct LSystem {}
impl LSystem {
    pub fn new() -> LSystem {
        LSystem {}
    }
}

impl Drawable for LSystem {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {}
}

impl Updatable for LSystem {
    fn update(&mut self, ctx: &UpdateContext) {}
}

impl EventHandler for LSystem {}

impl Runnable for LSystem {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
