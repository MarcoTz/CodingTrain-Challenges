use graphics::{
    colors::Rgba, Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext,
    Updatable, UpdateContext, WindowConfig,
};
use math::rand_between;
use piston_window::text::Text;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const VERT_SIZE: f64 = 20.0;
const EDGE_THICK: f64 = 2.0;

const NUM_VERTS: usize = 15;

mod brute_force;
mod genetic;
mod graph;
//use brute_force::BruteForceSolver;
use genetic::GeneticSolver;
use graph::Graph;

pub type Path = Vec<usize>;

pub trait Solver {
    fn current_best_path(&self) -> Path;
    fn current_best_weight(&self) -> f64;
    fn progress(&self) -> String;
    fn try_next(&mut self, graph: &Graph) -> Option<Path>;
}

pub struct TravellingSalesPerson {
    graph: Graph,
    text_color: Rgba,
    solver: Box<dyn Solver>,
}

impl TravellingSalesPerson {
    pub fn new() -> TravellingSalesPerson {
        TravellingSalesPerson {
            graph: Graph::new(),
            text_color: Rgba::random(),
            //solver: Box::new(BruteForceSolver::new()) as Box<dyn Solver>,
            solver: Box::new(GeneticSolver::new()) as Box<dyn Solver>,
        }
    }
}

impl Drawable for TravellingSalesPerson {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        Text::new_color(self.text_color.into(), 18)
            .draw_pos(
                &self.solver.progress(),
                [0.0, 20.0],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();

        self.graph.draw(ctx, gl);
    }
}

impl Updatable for TravellingSalesPerson {
    fn update(&mut self, _: &mut UpdateContext) {
        match self.solver.try_next(&self.graph) {
            None => (),
            Some(path) => self.graph.active_path = path,
        }
    }
}

impl EventHandler for TravellingSalesPerson {}

impl Runnable for TravellingSalesPerson {
    fn setup(&mut self, ctx: &mut SetupContext) {
        for _ in 0..NUM_VERTS {
            self.graph.add_vertex(
                rand_between(VERT_SIZE, ctx.window_width - VERT_SIZE),
                rand_between(VERT_SIZE, ctx.window_height - VERT_SIZE),
            )
        }
    }
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "TravellingSalesPerson".to_owned(),
        }
    }
}
