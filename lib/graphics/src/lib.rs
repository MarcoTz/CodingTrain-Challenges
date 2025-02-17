use gfx_device_gl::{CommandBuffer, Resources};
use piston_window::TextureContext;

const SCREENSHOT_DIR: &str = "./screenshots";

pub type TextureCtx = TextureContext<gfx_device_gl::Factory, Resources, CommandBuffer>;
pub type Graphics<'a> = gfx_graphics::GfxGraphics<'a, Resources, CommandBuffer>;

pub mod app;
pub mod colors;
pub mod poly_outline;
mod traits;
pub mod ui_elements;

pub use traits::drawable::{Drawable, DrawingContext};
pub use traits::eventhandler::{EventHandler, InputContext};
pub use traits::runnable::{Runnable, SetupContext, WindowConfig};
pub use traits::updatable::{Updatable, UpdateContext};

// reexports from piston graphics
pub use graphics::{
    circle_arc, clear, ellipse, image, line, polygon, rectangle, types::Color, DrawState,
    Transformed,
};
