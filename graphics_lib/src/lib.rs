pub mod app;
mod drawable;
mod eventhandler;
pub mod grid;
mod updatable;
pub mod vec2d;

pub use drawable::{Drawable, DrawingContext};
pub use eventhandler::{EventHandler, InputContext};
pub use updatable::{Updatable, UpdateContext};

use window::Size;

pub fn rand_between(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max - min)
}

pub struct SetupContext {
    pub window_height: f64,
    pub window_width: f64,
}

pub trait Runnable: Drawable + Updatable + EventHandler {
    fn window_size(&self) -> Size;
    fn setup(&mut self, _: &SetupContext) {}
}
