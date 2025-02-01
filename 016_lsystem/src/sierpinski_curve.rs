use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::{DrawingContext, Transformed};
use std::{collections::HashMap, f64::consts::PI, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum SierpinskiCurve {
    A,
    B,
    Plus,
    Minus,
}

impl Symbol for SierpinskiCurve {}

impl SierpinskiCurve {
    pub fn l_system() -> LSystem<SierpinskiCurve> {
        LSystem {
            axiom: vec![SierpinskiCurve::A],
            rules: HashMap::from([
                (
                    SierpinskiCurve::A,
                    vec![
                        SierpinskiCurve::B,
                        SierpinskiCurve::Minus,
                        SierpinskiCurve::A,
                        SierpinskiCurve::Minus,
                        SierpinskiCurve::B,
                    ],
                ),
                (
                    SierpinskiCurve::B,
                    vec![
                        SierpinskiCurve::A,
                        SierpinskiCurve::Plus,
                        SierpinskiCurve::B,
                        SierpinskiCurve::Plus,
                        SierpinskiCurve::A,
                    ],
                ),
            ]),
        }
    }
}

impl TurtleInstructor for SierpinskiCurve {
    fn start(&self, ctx: &DrawingContext, iter: u64) -> TurtleState {
        let mut st = TurtleState::new(
            ctx.id_trans().trans(ctx.args.window_size[0] / 2.0, 0.0),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        );
        st.len = ctx.args.window_size[0] / (2_f64.powi(iter as i32 + 1));
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            SierpinskiCurve::A | SierpinskiCurve::B => TurtleCommand::DrawLine,
            SierpinskiCurve::Plus => TurtleCommand::Turn(PI / 3.0),
            SierpinskiCurve::Minus => TurtleCommand::Turn(-PI / 3.0),
        }
    }
}
