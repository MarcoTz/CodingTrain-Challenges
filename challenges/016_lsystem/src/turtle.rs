use graphics::{line, Color, Drawable, DrawingContext, Graphics, Transformed};
pub struct Turtle {
    pub commands: Vec<Box<dyn TurtleInstructor>>,
    pub global_scale: f64,
    pub iteration: u64,
}

impl Turtle {
    pub fn new(cmds: Vec<Box<dyn TurtleInstructor>>) -> Turtle {
        Turtle {
            commands: cmds,
            global_scale: 1.0,
            iteration: 0,
        }
    }
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
    ChangeColor(Color),
    Multiple(Vec<TurtleCommand>),
    Push,
    Pop,
}

impl TurtleCommand {
    fn run(self, st: &mut TurtleState, gl: &mut Graphics) {
        match self {
            TurtleCommand::Walk => st.transform = st.transform.trans(0.0, st.len),
            TurtleCommand::Turn(angle) => st.transform = st.transform.rot_rad(angle),
            TurtleCommand::ChangeColor(col) => st.color = col,
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

pub trait TurtleInstructor {
    fn start(&self, ctx: &mut DrawingContext, num_commands: u64) -> TurtleState;
    fn command(&self) -> TurtleCommand;
}

impl Drawable for Turtle {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let mut state = self.commands.first().unwrap().start(ctx, self.iteration);
        state.len *= self.global_scale;
        for cmd in self.commands.iter() {
            cmd.command().run(&mut state, gl);
        }
    }
}
