use super::{DrawingContext, InputContext, Runnable, SetupContext, UpdateContext};
use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventSettings, Events},
    input::{ButtonArgs, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent},
    window::WindowSettings,
    ButtonEvent, MouseCursorEvent, ResizeEvent,
};
use window::Window;

pub struct App<T: Runnable> {
    gl: GlGraphics,
    window: GlutinWindow,
    events: Events,
    runnable: T,
    mouse_pos: [f64; 2],
}

impl<T: Runnable> App<T> {
    pub fn new(runnable: T) -> App<T> {
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
            mouse_pos: [0.0, 0.0],
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG, gl);
            let context = DrawingContext { context: c, args };
            self.runnable.draw(&context, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let size = self.window.size();
        let ctx = UpdateContext {
            window_width: size.width,
            window_height: size.height,
            args,
        };
        self.runnable.update(&ctx)
    }

    fn handle_input(&mut self, args: &ButtonArgs) {
        let size = self.window.size();
        let ctx = InputContext {
            args,
            window_width: size.width,
            window_height: size.height,
            mouse_pos: self.mouse_pos,
        };
        self.runnable.handle_input(&ctx)
    }

    pub fn run(&mut self) {
        let size = self.window.size();
        self.runnable.setup(&SetupContext {
            window_width: size.width,
            window_height: size.height,
        });
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
            if let Some(args) = e.mouse_cursor_args() {
                self.mouse_pos = args;
            }
            if let Some(args) = e.resize_args() {
                self.runnable.handle_resize(&args)
            }
        }
    }
}
