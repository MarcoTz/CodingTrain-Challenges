use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::Transformed;
use graphics_lib::DrawingContext;
use std::{collections::HashMap, f64::consts::PI, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Koch {
    F,
    Plus,
    Minus,
}

impl Symbol for Koch {}

impl Koch {
    pub fn l_system() -> LSystem<Koch> {
        LSystem {
            axiom: vec![Koch::F],
            rules: HashMap::from([(
                Koch::F,
                vec![
                    Koch::F,
                    Koch::Plus,
                    Koch::F,
                    Koch::Minus,
                    Koch::F,
                    Koch::Minus,
                    Koch::F,
                    Koch::Plus,
                    Koch::F,
                ],
            )]),
        }
    }
}

impl TurtleInstructor for Koch {
    fn start(&self, ctx: &DrawingContext, iter: u64) -> TurtleState {
        let mut st = TurtleState::new(
            ctx.id_trans().trans(ctx.args.window_size[0] / 2.0, 0.0),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        );
        st.len = ctx.args.window_size[0] / (3_f64.powi(iter as i32));
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            Koch::F => TurtleCommand::DrawLine,
            Koch::Plus => TurtleCommand::Turn(-PI / 2.0),
            Koch::Minus => TurtleCommand::Turn(PI / 2.0),
        }
    }
}
