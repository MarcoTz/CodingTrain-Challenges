use graphics_lib::{app::App, drawable::Drawable};
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

struct Point {
    x: f64,
    y: f64,
}

struct Square {
    size: f64,
    rotation: f64,
    pos: Point,
}

impl Drawable for Square {
    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, self.size);
        let rotation = self.rotation;
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(center_x + self.pos.x, center_y + self.pos.y)
                .rot_rad(rotation)
                .trans(-self.size / 2.0, -self.size / 2.0);
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

pub fn run_window() {
    let mut app = App::new();
    app.add_object(Square {
        pos: Point {
            x: -100.0,
            y: -100.0,
        },
        size: 100.0,
        rotation: 0.0,
    });
    app.add_object(Square {
        pos: Point { x: 0.0, y: 0.0 },
        size: 50.0,
        rotation: 0.0,
    });
    app.run();
}
