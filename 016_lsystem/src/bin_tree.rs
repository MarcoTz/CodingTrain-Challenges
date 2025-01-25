use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::Transformed;
use graphics_lib::DrawingContext;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum BinTree {
    Zero,
    One,
    Open,
    Close,
}

impl Symbol for BinTree {}

impl BinTree {
    pub fn l_system() -> LSystem<BinTree> {
        LSystem {
            axiom: vec![BinTree::Zero],
            rules: HashMap::from([
                (BinTree::One, vec![BinTree::One, BinTree::One]),
                (
                    BinTree::Zero,
                    vec![
                        BinTree::One,
                        BinTree::Open,
                        BinTree::Zero,
                        BinTree::Close,
                        BinTree::Zero,
                    ],
                ),
            ]),
        }
    }
}

impl TurtleInstructor for BinTree {
    fn start(&self, ctx: &DrawingContext, iter: u64) -> TurtleState {
        let mut st = TurtleState::new(
            ctx.id_trans()
                .trans(ctx.args.window_size[0] / 2.0, ctx.args.window_size[1])
                .rot_rad(-PI),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        );
        st.len = 2.5 * ctx.args.window_size[1] / (iter as f64);
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            BinTree::Zero => TurtleCommand::DrawLine,
            BinTree::One => TurtleCommand::DrawLine,
            BinTree::Open => {
                TurtleCommand::Multiple(vec![TurtleCommand::Push, TurtleCommand::Turn(-PI / 4.0)])
            }
            BinTree::Close => {
                TurtleCommand::Multiple(vec![TurtleCommand::Pop, TurtleCommand::Turn(PI / 4.0)])
            }
        }
    }
}
