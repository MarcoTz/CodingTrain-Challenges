use super::{DrawingContext, InputContext, Runnable, SetupContext, UpdateContext};
use graphics::clear;
use opengl_graphics::OpenGL;
use piston::{
    event_loop::{EventSettings, Events},
    input::{ButtonArgs, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent},
    window::WindowSettings,
    ButtonEvent, Event, MouseCursorEvent, ResizeEvent, Window,
};
use piston_window::PistonWindow;

pub struct App<T: Runnable> {
    window: PistonWindow,
    events: Events,
    runnable: T,
    mouse_pos: [f64; 2],
}

impl<T: Runnable> App<T> {
    pub fn new(runnable: T) -> App<T> {
        let config = runnable.config();
        let opengl = OpenGL::V3_2;
        let window: PistonWindow = WindowSettings::new(config.title, [config.width, config.height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let events = Events::new(EventSettings::new());
        App {
            window,
            events,
            runnable,
            mouse_pos: [0.0, 0.0],
        }
    }

    fn render(&mut self, e: &Event, args: &RenderArgs) {
        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let ctx = self.window.create_texture_context();
        self.window.draw_2d(e, |c, gl, _| {
            clear(BG, gl);
            let context = DrawingContext {
                context: c,
                args,
                texture_context: ctx,
            };
            self.runnable.draw(&context, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let size = self.window.size();
        let ctx = UpdateContext {
            window_width: size.width,
            window_height: size.height,
            mouse_pos: self.mouse_pos,
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
            if let Some(args) = e.clone().render_args() {
                self.render(&e, &args);
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
