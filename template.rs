use graphics::{
    Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable, UpdateContext,
    WindowConfig,
};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

pub struct NAME {}

impl NAME {
    pub fn new() -> NAME {
        NAME {}
    }
}

impl Drawable for NAME {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {}
}

impl Updatable for NAME {
    fn update(&mut self, ctx: &mut UpdateContext) {}
}

impl EventHandler for NAME {}

impl Runnable for NAME {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "NAME".to_owned(),
        }
    }
}
