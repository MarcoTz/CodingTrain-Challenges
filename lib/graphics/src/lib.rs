use gfx_device_gl::{CommandBuffer, Resources};
use piston_window::TextureContext;

const SCREENSHOT_DIR: &str = "./screenshots";

pub type TextureCtx = TextureContext<gfx_device_gl::Factory, Resources, CommandBuffer>;
pub type Graphics<'a> = gfx_graphics::GfxGraphics<'a, Resources, CommandBuffer>;

pub mod app;
mod drawable;
mod eventhandler;
mod runnable;
mod updatable;

pub use drawable::{Drawable, DrawingContext};
pub use eventhandler::{EventHandler, InputContext};
pub use runnable::{Runnable, SetupContext, WindowConfig};
pub use updatable::{Updatable, UpdateContext};

// reexports from piston graphics
pub use graphics::{
    circle_arc, clear, ellipse, line, polygon, rectangle, types::Color, DrawState, Transformed,
};
