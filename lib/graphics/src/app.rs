use super::{DrawingContext, InputContext, Runnable, SetupContext, UpdateContext};
use graphics::clear;
use opengl_graphics::OpenGL;
use piston::{
    event_loop::{EventSettings, Events},
    input::{ButtonArgs, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent},
    window::WindowSettings,
    ButtonEvent, Event, MouseCursorEvent, ResizeEvent, Window,
};
use piston_window::{G2dTextureContext, Glyphs, PistonWindow, TextureSettings};

pub struct App<T: Runnable> {
    window: PistonWindow,
    events: Events,

    runnable: T,
    mouse_pos: [f64; 2],
    glyphs: Glyphs,
    texture_context: G2dTextureContext,
}

impl<T: Runnable> App<T> {
    pub fn new(runnable: T) -> App<T> {
        let config = runnable.config();

        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new(config.title, [config.width, config.height])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();

        let events = Events::new(EventSettings::new());

        let glyphs = Glyphs::from_bytes(
            include_bytes!("font.ttf",),
            window.create_texture_context(),
            TextureSettings::new(),
        )
        .unwrap();

        let context = window.create_texture_context();
        App {
            window,
            events,
            runnable,
            mouse_pos: [0.0, 0.0],
            glyphs,
            texture_context: context,
        }
    }

    fn render(&mut self, e: &Event, args: &RenderArgs) {
        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.window.draw_2d(e, |c, gl, device| {
            clear(BG, gl);
            let mut context = DrawingContext {
                context: &c,
                args,
                glyphs: &mut self.glyphs,
                texture_context: &mut self.texture_context,
            };
            self.runnable.draw(&mut context, gl);
            self.glyphs.factory.encoder.flush(device);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let size = self.window.size();
        let mut ctx = UpdateContext {
            window_width: size.width,
            window_height: size.height,
            mouse_pos: self.mouse_pos,
            args,
            texture_context: &mut self.texture_context,
        };
        self.runnable.update(&mut ctx)
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
        self.runnable.setup(&mut SetupContext {
            window_width: size.width,
            window_height: size.height,
            texture_context: &mut self.texture_context,
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
