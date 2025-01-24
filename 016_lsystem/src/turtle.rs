use graphics::{line, types::Color, Transformed};
use graphics_lib::{Drawable, DrawingContext};
use opengl_graphics::GlGraphics;
pub struct Turtle<T: TurtleInstructor> {
    pub commands: Vec<T>,
    pub global_scale: f64,
}

#[derive(Clone)]
pub struct TurtleState {
    pub transform: [[f64; 3]; 2],
    pub color: Color,
    pub radius: f64,
    pub state_stack: Vec<TurtleState>,
    pub len: f64,
}

impl TurtleState {
    pub fn new(transform: [[f64; 3]; 2], color: Color, radius: f64) -> TurtleState {
        TurtleState {
            transform,
            color,
            radius,
            state_stack: vec![],
            len: 100.0,
        }
    }
}

pub enum TurtleCommand {
    Walk,
    Turn(f64),
    DrawLine,
    Scale(f64, f64),
    ChangeColor(Color),
    ChangeRadius(f64),
    Multiple(Vec<TurtleCommand>),
    Push,
    Pop,
}

impl TurtleCommand {
    fn run(self, st: &mut TurtleState, gl: &mut GlGraphics) {
        match self {
            TurtleCommand::Walk => st.transform = st.transform.trans(0.0, st.len),
            TurtleCommand::Turn(angle) => st.transform = st.transform.rot_rad(angle),
            TurtleCommand::Scale(sc_x, sc_y) => st.transform = st.transform.scale(sc_x, sc_y),
            TurtleCommand::ChangeColor(col) => st.color = col,
            TurtleCommand::ChangeRadius(r) => st.radius = r,
            TurtleCommand::DrawLine => {
                line(
                    st.color,
                    st.radius,
                    [0.0, 0.0, 0.0, st.len],
                    st.transform,
                    gl,
                );
                st.transform = st.transform.trans(0.0, st.len);
            }
            TurtleCommand::Multiple(cmds) => {
                for cmd in cmds.into_iter() {
                    cmd.run(st, gl)
                }
            }
            TurtleCommand::Push => st.state_stack.push(st.clone()),
            TurtleCommand::Pop => *st = st.state_stack.pop().unwrap(),
        }
    }
}

pub trait TurtleInstructor: Sized {
    fn start(ctx: &DrawingContext, cmds: &[Self]) -> TurtleState;
    fn command(&self) -> TurtleCommand;
}

impl<T: TurtleInstructor> Drawable for Turtle<T> {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let mut state = T::start(ctx, &self.commands);
        state.len *= self.global_scale;
        for cmd in self.commands.iter() {
            cmd.command().run(&mut state, gl);
        }
    }
}
