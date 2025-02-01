use graphics::{
    rectangle, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable, Updatable,
    WindowConfig,
};
use piston::{Button, ButtonState, Key};

const WINDOW_WIDTH: f64 = 1000.0;
const WINDOW_HEIGHT: f64 = 1000.0;

pub struct Menger {
    iteration: u32,
}

impl Menger {
    pub fn new() -> Menger {
        Menger { iteration: 0 }
    }
}

impl Default for Menger {
    fn default() -> Menger {
        Menger::new()
    }
}

impl Drawable for Menger {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let transform = ctx.center_trans();
        let mut width = ctx.args.window_size[0] / 1.5;
        width = width + ((3 - width as u64 % 3) as f64);
        let mut height = ctx.args.window_size[1] / 1.5;
        height = height + ((3 - height as u64 % 3) as f64);

        let start_x = -width / 2.0;
        let start_y = -height / 2.0;

        let white = [1.0, 1.0, 1.0, 1.0];
        let black = [0.0, 0.0, 0.0, 1.0];

        rectangle(white, [start_x, start_y, width, height], transform, gl);
        for i in 1..=self.iteration {
            let num_rows = 3_u32.pow(i);
            let square_width = width / num_rows as f64;
            let square_height = height / num_rows as f64;
            for x in 0..=num_rows {
                for y in 0..=num_rows {
                    if (y) % 3 != 1 || (x) % 3 != 1 || ((x * y) % 3) != 1 {
                        continue;
                    }

                    let pos_x = start_x + x as f64 * square_width;
                    let pos_y = start_y + y as f64 * square_height;
                    rectangle(
                        black,
                        [pos_x, pos_y, square_width, square_height],
                        transform,
                        gl,
                    );
                }
            }
        }
    }
}

impl EventHandler for Menger {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release {
            return;
        }
        let key = if let Button::Keyboard(key) = ctx.args.button {
            key
        } else {
            return;
        };
        match key {
            Key::Space => self.iteration += 1,
            Key::Escape | Key::Q => std::process::exit(0),
            _ => (),
        }
    }
}

impl Updatable for Menger {}

impl Runnable for Menger {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Menger Sponge".to_owned(),
        }
    }
}
