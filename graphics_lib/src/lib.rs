use std::process::{Command, ExitStatus};

const SCREENSHOT_DIR: &str = "./screenshots";

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
    fn screenshot(&self) {
        let title = self.config().title.replace(" ", "");
        let date_str = chrono::Local::now().format("%Y%m%d_%H%M%s");
        let res = Command::new("scrot")
            .arg("--focused")
            .arg("-F")
            .arg(format!("{SCREENSHOT_DIR}/{title}{date_str}.png"))
            .status();
        match res {
            Ok(status) => println!("took screenshot exit status {status}"),
            Err(err) => println!("could not take screenshot: {err}"),
        }
    }
}
