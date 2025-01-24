use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::Transformed;
use graphics_lib::DrawingContext;
use std::{collections::HashMap, f64::consts::PI, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Algae {
    A,
    B,
}

impl Algae {
    pub fn l_system() -> LSystem<Algae> {
        LSystem {
            axiom: vec![Algae::A],
            rules: HashMap::from([
                (Algae::A, vec![Algae::A, Algae::B]),
                (Algae::B, vec![Algae::A]),
            ]),
        }
    }
}

impl Symbol for Algae {}

impl TurtleInstructor for Algae {
    fn start(ctx: &DrawingContext, _: &[Self]) -> TurtleState {
        TurtleState::new(
            ctx.id_trans().trans(
                2.0 * ctx.args.window_size[0] / 3.0,
                ctx.args.window_size[1] / 2.0,
            ),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        )
    }

    fn command(&self) -> TurtleCommand {
        match self {
            Algae::A => {
                TurtleCommand::Multiple(vec![TurtleCommand::Turn(PI / 2.0), TurtleCommand::Walk])
            }
            Algae::B => TurtleCommand::Multiple(vec![
                TurtleCommand::ChangeColor([rand::random(), rand::random(), rand::random(), 1.0]),
                TurtleCommand::DrawLine,
            ]),
        }
    }
}
