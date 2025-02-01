use super::{
    l_system::{LSystem, Symbol},
    turtle::{TurtleCommand, TurtleInstructor, TurtleState},
};
use graphics::{DrawingContext, Transformed};
use std::{collections::HashMap, f64::consts::PI, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Sierpinski {
    F,
    G,
    Plus,
    Minus,
}

impl Symbol for Sierpinski {}

impl Sierpinski {
    pub fn l_system() -> LSystem<Sierpinski> {
        LSystem {
            axiom: vec![
                Sierpinski::F,
                Sierpinski::Minus,
                Sierpinski::G,
                Sierpinski::Minus,
                Sierpinski::G,
            ],
            rules: HashMap::from([
                (
                    Sierpinski::F,
                    vec![
                        Sierpinski::F,
                        Sierpinski::Minus,
                        Sierpinski::G,
                        Sierpinski::Plus,
                        Sierpinski::F,
                        Sierpinski::Plus,
                        Sierpinski::G,
                        Sierpinski::Minus,
                        Sierpinski::F,
                    ],
                ),
                (Sierpinski::G, vec![Sierpinski::G, Sierpinski::G]),
            ]),
        }
    }
}

impl TurtleInstructor for Sierpinski {
    fn start(&self, ctx: &DrawingContext, iter: u64) -> TurtleState {
        let mut st = TurtleState::new(
            ctx.id_trans().trans(ctx.args.window_size[0], 0.0),
            [1.0, 1.0, 1.0, 1.0],
            2.0,
        );
        st.len = ctx.args.window_size[1].min(ctx.args.window_size[0]) / (2_f64.powi(iter as i32));
        st
    }

    fn command(&self) -> TurtleCommand {
        match self {
            Sierpinski::F | Sierpinski::G => TurtleCommand::DrawLine,
            Sierpinski::Plus => TurtleCommand::Turn(-2.0 * PI / 3.0),
            Sierpinski::Minus => TurtleCommand::Turn(2.0 * PI / 3.0),
        }
    }
}
