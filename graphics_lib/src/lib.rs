use gfx_device_gl::{CommandBuffer, Resources};
use piston_window::TextureContext;

const SCREENSHOT_DIR: &str = "./screenshots";

pub type TextureCtx = TextureContext<gfx_device_gl::Factory, Resources, CommandBuffer>;
pub type Graphics<'a> = gfx_graphics::GfxGraphics<'a, Resources, CommandBuffer>;

pub mod app;
mod drawable;
mod eventhandler;
pub mod grid;
mod runnable;
mod updatable;
pub mod vec2d;

pub use drawable::{Drawable, DrawingContext};
pub use eventhandler::{EventHandler, InputContext};
pub use runnable::{Runnable, SetupContext, WindowConfig};
pub use updatable::{Updatable, UpdateContext};

pub fn rand_between(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max - min)
}
