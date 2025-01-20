use graphics::rectangle;
use graphics_lib::{app::App, Drawable, DrawingContext, InputHandler, Runnable};
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonArgs, ButtonState, Key, RenderArgs, UpdateArgs};
use window::Size;

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

impl Drawable for Menger {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.center_trans();
        let mut width = ctx.args.window_size[0] as f64 / 1.5;
        width = width + ((3 - width as u64 % 3) as f64);
        let mut height = ctx.args.window_size[1] as f64 / 1.5;
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

impl InputHandler for Menger {
    fn handle(&mut self, args: &ButtonArgs) {
        if args.state != ButtonState::Release {
            return;
        }
        let key = if let Button::Keyboard(key) = args.button {
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

impl Runnable for Menger {
    fn window_size(&self) -> Size {
        Size {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        }
    }

    fn to_draw(&self) -> Vec<&dyn Drawable> {
        vec![self]
    }
    fn handlers(&mut self) -> Vec<&mut dyn InputHandler> {
        vec![self]
    }
}
