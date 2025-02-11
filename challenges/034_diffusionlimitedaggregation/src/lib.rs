use graphics::{
    Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const NUM_WALKERS: usize = 100;
const NUM_AGG: usize = 1;
const WALKER_SIZE: f64 = 20.0;
const AGG_SIZE: f64 = 10.0;
const STEP_SIZE: f64 = 10.0;

mod aggregator;
mod trace;
mod walker;
use aggregator::Aggregator;
//use trace::Trace;
use walker::Walker;

pub struct DiffAgg {
    walkers: Vec<Walker>,
    //    traces: Vec<Trace>,
    aggregators: Vec<Aggregator>,
}

impl DiffAgg {
    pub fn new() -> DiffAgg {
        DiffAgg {
            walkers: vec![],
            //            traces: vec![],
            aggregators: vec![],
        }
    }

    fn collect_agg(&mut self) {
        let mut to_remove = vec![];
        'walker_loop: for (walker_ind, walker) in self.walkers.iter().enumerate() {
            for (agg_ind, agg) in self.aggregators.iter().enumerate() {
                if agg.walker_hits(&walker.pos) {
                    to_remove.push((agg_ind, walker_ind));
                    continue 'walker_loop;
                }
            }
        }

        to_remove.reverse();
        for (agg_ind, walker_ind) in to_remove {
            let old_walker = self.walkers.remove(walker_ind);
            self.aggregators[agg_ind].aggregated.push(old_walker.pos);
            //            self.traces.push(old_walker.trace);
        }
    }

    fn fill_walkers(&mut self, window_width: f64, window_height: f64) {
        while self.walkers.len() < NUM_WALKERS {
            let mut new_walker = Walker::new();
            new_walker.pos = Vec2D::new(
                rand_between(0.0, window_width),
                rand_between(0.0, window_height),
            );
            self.walkers.push(new_walker);
        }
    }
}

impl Drawable for DiffAgg {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for walker in self.walkers.iter() {
            walker.draw(ctx, gl)
        }

        /*        for trace in self.traces.iter() {
            trace.draw(ctx, gl);
        }*/

        for agg in self.aggregators.iter() {
            agg.draw(ctx, gl);
        }
    }
}

impl Updatable for DiffAgg {
    fn update(&mut self, ctx: &mut UpdateContext) {
        for walker in self.walkers.iter_mut() {
            for _ in 0..10 {
                walker.update(ctx);
            }
        }
        self.collect_agg();
        self.fill_walkers(ctx.window_width, ctx.window_height);
    }
}

impl EventHandler for DiffAgg {}

impl Runnable for DiffAgg {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.fill_walkers(ctx.window_width, ctx.window_height);

        for _ in 0..NUM_AGG {
            let mut new_agg = Aggregator::new();
            new_agg.pos = Vec2D::new(
                ctx.window_width / 2.0,
                ctx.window_height / 2.0, //rand_between(0.0, ctx.window_width),
                                         //rand_between(0.0, ctx.window_height),
            );
            self.aggregators.push(new_agg);
        }
    }
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "DiffAgg".to_owned(),
        }
    }
}
