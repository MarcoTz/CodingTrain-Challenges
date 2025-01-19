use super::drawable::Drawable;
use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    window: GlutinWindow,
    events: Events,
    objects: Vec<Box<dyn Drawable>>,
}

impl App {
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("spinning-square", [200, 200])
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

    pub fn add_object<T: 'static + Drawable>(&mut self, obj: T) {
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

    pub fn run(&mut self) {
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
