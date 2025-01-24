use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics_lib::DrawingContext;
use std::{collections::HashMap, f64::consts::PI, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Dragon {
    F,
    G,
    Plus,
    Minus,
}

impl Symbol for Dragon {}

impl Dragon {
    pub fn l_system() -> LSystem<Dragon> {
        LSystem {
            axiom: vec![Dragon::F],
            rules: HashMap::from([
                (Dragon::F, vec![Dragon::F, Dragon::Plus, Dragon::G]),
                (Dragon::G, vec![Dragon::F, Dragon::Minus, Dragon::G]),
            ]),
        }
    }
}

impl TurtleInstructor for Dragon {
    fn start(ctx: &DrawingContext, cmds: &[Self]) -> TurtleState {
        let mut st = TurtleState::new(ctx.center_trans(), [1.0, 1.0, 1.0, 1.0], 2.0);
        st.len = ctx.args.window_size[0] / (cmds.len() as f64 / 5.0);
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            Dragon::F | Dragon::G => TurtleCommand::DrawLine,
            Dragon::Plus => TurtleCommand::Turn(-PI / 2.0),
            Dragon::Minus => TurtleCommand::Turn(PI / 2.0),
        }
    }
}
