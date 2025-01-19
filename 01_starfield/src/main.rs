use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    window: Window,
    events: Events,
    objects: Vec<Box<dyn Drawable>>,
}

impl App {
    fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("spinning-square", [200, 200])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let events = Events::new(EventSettings::new());
        App {
            window,
            events,
            gl: GlGraphics::new(opengl),
            objects: vec![],
        }
    }

    fn add_object<T: 'static + Drawable>(&mut self, obj: T) {
        self.objects.push(Box::new(obj))
    }

    fn render(&mut self, args: &RenderArgs) {
        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |_, gl| {
            clear(BG, gl);
        });

        for object in self.objects.iter() {
            object.draw(args, &mut self.gl);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        for object in self.objects.iter_mut() {
            object.update(&args);
        }
    }

    fn run(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}

trait Drawable {
    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics);
    fn update(&mut self, args: &UpdateArgs);
}

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

fn main() {
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
