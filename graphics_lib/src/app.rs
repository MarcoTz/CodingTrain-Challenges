use super::Object;
use glutin_window::GlutinWindow;
use graphics::{clear, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    input::{ButtonArgs, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent},
    window::WindowSettings,
    ButtonEvent,
};

pub struct App {
    gl: GlGraphics,
    window: GlutinWindow,
    events: Events,
    objects: Vec<Box<dyn Object>>,
}

impl App {
    pub fn new(width: f64, height: f64) -> App {
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("spinning-square", [width, height])
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

    pub fn add_object<T: 'static + Object>(&mut self, obj: T) {
        self.objects.push(Box::new(obj))
    }

    fn render(&mut self, args: &RenderArgs) {
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(center_x, center_y);
            for object in self.objects.iter() {
                object.draw(args, gl, transform);
            }

            clear(BG, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for object in self.objects.iter_mut() {
            object.update(&args);
        }
    }

    fn handle_input(&mut self, args: &ButtonArgs) {
        for object in self.objects.iter_mut() {
            object.handle(args);
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
            if let Some(args) = e.button_args() {
                self.handle_input(&args);
            }
        }
    }
}
