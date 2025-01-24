use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::Transformed;
use graphics_lib::DrawingContext;
use std::{collections::HashMap, f64::consts::PI, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Plant {
    X,
    F,
    Plus,
    Minus,
    Open,
    Close,
}

impl Symbol for Plant {}

impl Plant {
    pub fn l_system() -> LSystem<Plant> {
        LSystem {
            axiom: vec![Plant::X],
            rules: HashMap::from([
                (
                    Plant::X,
                    vec![
                        Plant::F,
                        Plant::Plus,
                        Plant::Open,
                        Plant::Open,
                        Plant::X,
                        Plant::Close,
                        Plant::Minus,
                        Plant::X,
                        Plant::Close,
                        Plant::Minus,
                        Plant::F,
                        Plant::Open,
                        Plant::Minus,
                        Plant::F,
                        Plant::X,
                        Plant::Close,
                        Plant::Plus,
                        Plant::X,
                    ],
                ),
                (Plant::F, vec![Plant::F, Plant::F]),
            ]),
        }
    }
}

impl TurtleInstructor for Plant {
    fn start(ctx: &DrawingContext, cmds: &[Self]) -> TurtleState {
        let mut st = TurtleState::new(
            ctx.id_trans()
                .trans(ctx.args.window_size[0] / 2.0, ctx.args.window_size[1])
                .rot_rad(PI),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        );
        st.len = 10.0 * ctx.args.window_size[1]
            / (cmds
                .iter()
                .filter(|pl| matches!(pl, Plant::F))
                .collect::<Vec<&Self>>()
                .len() as f64);
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            Plant::F => TurtleCommand::DrawLine,
            Plant::Plus => TurtleCommand::Turn(-5.0 * PI / 36.0),
            Plant::Minus => TurtleCommand::Turn(5.0 * PI / 36.0),
            Plant::Open => TurtleCommand::Push,
            Plant::Close => TurtleCommand::Pop,
            Plant::X => TurtleCommand::Multiple(vec![]),
        }
    }
}
