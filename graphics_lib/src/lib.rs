use std::process::{Command, ExitStatus};

const SCREENSHOT_DIR: &str = "./screnshots";

pub mod app;
mod drawable;
mod eventhandler;
pub mod grid;
mod updatable;
pub mod vec2d;

pub use drawable::{Drawable, DrawingContext};
pub use eventhandler::{EventHandler, InputContext};
pub use updatable::{Updatable, UpdateContext};

pub fn rand_between(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max - min)
}

pub struct SetupContext {
    pub window_height: f64,
    pub window_width: f64,
}

pub struct WindowConfig {
    pub width: f64,
    pub height: f64,
    pub title: String,
}

pub trait Runnable: Drawable + Updatable + EventHandler {
    fn config(&self) -> WindowConfig;
    fn setup(&mut self, _: &SetupContext) {}
    fn screnshot(&self) -> Result<ExitStatus, std::io::Error> {
        let title = self.config().title;
        Command::new("scrot")
            .arg("--focused")
            .arg("-F")
            .arg(format!("{SCREENSHOT_DIR}/{title}.png"))
            .status()
    }
}
