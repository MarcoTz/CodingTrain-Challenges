use super::{DrawingContext, Runnable, UpdateContext};
use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    input::{ButtonArgs, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent},
    window::WindowSettings,
    ButtonEvent,
};
use window::Window;

pub struct App {
    gl: GlGraphics,
    window: GlutinWindow,
    events: Events,
    runnable: Box<dyn Runnable>,
}

impl App {
    pub fn new(runnable: Box<dyn Runnable>) -> App {
        let size = runnable.window_size();
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow =
            WindowSettings::new("spinning-square", [size.width, size.height])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();
        let events = Events::new(EventSettings::new());
        App {
            window,
            events,
            gl: GlGraphics::new(opengl),
            runnable,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG, gl);
            let context = DrawingContext { context: c, args };
            for object in self.runnable.to_draw().iter() {
                object.draw(&context, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let size = self.window.size();
        let ctx = UpdateContext {
            window_width: size.width,
            window_height: size.height,
            args,
        };
        for object in self.runnable.to_update().iter_mut() {
            object.update(&ctx);
        }
    }

    fn handle_input(&mut self, args: &ButtonArgs) {
        for object in self.runnable.handlers().iter_mut() {
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
