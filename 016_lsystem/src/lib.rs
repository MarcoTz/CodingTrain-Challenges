use graphics_lib::{
    Drawable, DrawingContext, EventHandler, InputContext, Runnable, Updatable, UpdateContext,
};
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonState, Key, Size};

mod algae;
mod bin_tree;
mod cantor;
mod dragon;
mod koch;
mod plant;
mod sierpinski;
mod sierpinski_curve;

mod l_system;
mod systems;
mod turtle;
use systems::System;
use turtle::Turtle;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

pub struct SystemRunner {
    current_system: usize,
    turtle: Turtle,
    paused: bool,
    systems: Vec<System>,
}

impl SystemRunner {
    pub fn new() -> SystemRunner {
        let system = System::default();
        SystemRunner {
            turtle: Turtle::new(system.axiom()),
            current_system: 0,
            paused: true,
            systems: System::all(),
        }
    }
}

impl Drawable for SystemRunner {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        self.turtle.draw(ctx, gl)
    }
}

impl Updatable for SystemRunner {
    fn update(&mut self, _: &UpdateContext) {
        if self.paused {
            return;
        }
        self.systems[self.current_system].next_iter();
        self.turtle.commands = self.systems[self.current_system].commands();
        self.turtle.iteration += 1;
        self.paused = true;
    }
}

impl EventHandler for SystemRunner {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release {
            return;
        }
        if ctx.args.button == Button::Keyboard(Key::Space) {
            self.paused = false
        }

        if ctx.args.button == Button::Keyboard(Key::Plus)
            || ctx.args.button == Button::Keyboard(Key::Equals)
        {
            self.turtle.global_scale += 0.1;
        }

        if ctx.args.button == Button::Keyboard(Key::Minus) {
            self.turtle.global_scale -= 0.1;
        }

        if ctx.args.button == Button::Keyboard(Key::N) {
            self.current_system += 1;
            self.current_system = self.current_system % self.systems.len();
            self.systems[self.current_system].reset();
            self.turtle.iteration = 0;
            self.turtle.commands = self.systems[self.current_system].commands();
            println!("current system {}", self.systems[self.current_system]);
        }
    }
}

impl Runnable for SystemRunner {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
