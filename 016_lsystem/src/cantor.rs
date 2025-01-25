use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::Transformed;
use graphics_lib::DrawingContext;
use std::{collections::HashMap, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Cantor {
    A,
    B,
}

impl Symbol for Cantor {}

impl Cantor {
    pub fn l_system() -> LSystem<Cantor> {
        LSystem {
            axiom: vec![Cantor::A],
            rules: HashMap::from([
                (Cantor::A, vec![Cantor::A, Cantor::B, Cantor::A]),
                (Cantor::B, vec![Cantor::B, Cantor::B, Cantor::B]),
            ]),
        }
    }
}

impl TurtleInstructor for Cantor {
    fn start(&self, ctx: &DrawingContext, iter: u64) -> TurtleState {
        let mut st = TurtleState::new(
            ctx.id_trans().trans(ctx.args.window_size[0] / 2.0, 0.0),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        );
        st.len = ctx.args.window_size[1] / (3_f64.powi(iter as i32));
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            Cantor::A => TurtleCommand::DrawLine,
            Cantor::B => TurtleCommand::Walk,
        }
    }
}
