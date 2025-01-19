pub mod app;
pub mod drawable;
pub mod input_handler;
pub mod point;

use drawable::Drawable;
use input_handler::InputHandler;

pub trait Object: Drawable + InputHandler {}

impl<T: Drawable + InputHandler> Object for T {}
