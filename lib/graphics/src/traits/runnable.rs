use super::{drawable::Drawable, eventhandler::EventHandler, updatable::Updatable};
use crate::SCREENSHOT_DIR;
use piston_window::G2dTextureContext;
use std::process::Command;

pub struct SetupContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub texture_context: &'a mut G2dTextureContext,
}

pub struct WindowConfig {
    pub width: f64,
    pub height: f64,
    pub title: String,
}

pub trait Runnable: Drawable + Updatable + EventHandler {
    fn config(&self) -> WindowConfig;
    fn setup(&mut self, _: &mut SetupContext) {}
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
