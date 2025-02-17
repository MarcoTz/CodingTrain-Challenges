use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::DrawingContext;
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
    fn start(&self, ctx: &mut DrawingContext, iter: u64) -> TurtleState {
        let mut st = TurtleState::new(ctx.center_trans(), [1.0, 1.0, 1.0, 1.0], 2.0);
        st.len = ctx.args.window_size[0].min(ctx.args.window_size[1]) / (1.5_f64).powi(iter as i32);
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
