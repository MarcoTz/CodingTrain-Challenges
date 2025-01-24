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
mod turtle;
use l_system::{LSystem, Symbol};
use turtle::{Turtle, TurtleInstructor};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

type Alphabet = koch::Koch;

pub struct SystemRunner<T: Symbol + TurtleInstructor> {
    system: LSystem<T>,
    turtle: Turtle<T>,
    paused: bool,
}

impl SystemRunner<Alphabet> {
    pub fn new() -> SystemRunner<Alphabet> {
        let system = Alphabet::l_system();
        SystemRunner {
            turtle: Turtle {
                commands: system.axiom.clone(),

                global_scale: 1.0,
            },
            system,
            paused: true,
        }
    }
}

impl<T: Symbol + TurtleInstructor> Drawable for SystemRunner<T> {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        self.turtle.draw(ctx, gl)
    }
}

impl<T: Symbol + TurtleInstructor> Updatable for SystemRunner<T> {
    fn update(&mut self, _: &UpdateContext) {
        if self.paused {
            return;
        }
        self.turtle.commands = self.system.next(&self.turtle.commands);
        self.paused = true;
    }
}

impl<T: Symbol + TurtleInstructor> EventHandler for SystemRunner<T> {
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
    }
}

impl<T: Symbol + TurtleInstructor> Runnable for SystemRunner<T> {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
