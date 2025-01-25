use graphics::{ellipse, line};
use graphics_lib::{
    rand_between, vec2d::Vec2D, Drawable, DrawingContext, EventHandler, Runnable, SetupContext,
    Updatable, UpdateContext,
};
use opengl_graphics::GlGraphics;
use piston::{ResizeArgs, Size};
use std::time::Instant;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const NUM_POINTS: usize = 200;
const DIST_NEXT: f64 = 50.0;
const INF_RADIUS: f64 = 18.0 * DIST_NEXT;
const KILL_DIST: f64 = 20.0;

const MAX_THICK: f64 = 10.0;
const MIN_THICK: f64 = 1.0;

struct TreeNode {
    pos: Vec2D,
    next: Option<Vec2D>,
}

impl TreeNode {
    pub fn new(x: f64, y: f64) -> TreeNode {
        TreeNode {
            pos: Vec2D::new(x, y),
            next: None,
        }
    }

    fn avg_distance(&self, pts: &[&Vec2D]) -> Vec2D {
        let mut dist = Vec2D::default();
        for pt in pts.iter() {
            let mut next = **pt - self.pos;
            next.set_abs(1.0);
            dist += next;
        }
        dist.set_abs(1.0);
        dist
    }
}

pub struct SpaceColonization {
    attraction_points: Vec<Vec2D>,
    nodes: Vec<TreeNode>,
}

impl SpaceColonization {
    pub fn new() -> SpaceColonization {
        SpaceColonization {
            attraction_points: Vec::with_capacity(NUM_POINTS),
            nodes: vec![],
        }
    }

    fn closest_node(&self, pt: &Vec2D) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .min_by(|(_, node1), (_, node2)| {
                node1.pos.dist(pt).partial_cmp(&node2.pos.dist(pt)).unwrap()
            })
            .filter(|(_, node)| node.pos.dist(pt) < INF_RADIUS)
            .map(|(ind, _)| ind)
    }

    fn generate_attraction(&mut self, window_width: f64, window_height: f64) {
        self.attraction_points.clear();
        for _ in 0..NUM_POINTS {
            let mut new_point = Vec2D {
                x: rand::random::<f64>() * window_width,
                y: rand_between(0.0, 2.0 * window_height / 3.0),
            };
            while self.attraction_points.contains(&new_point) {
                new_point = Vec2D {
                    x: rand::random::<f64>() * window_width,
                    y: rand_between(0.0, 2.0 * window_height / 3.0),
                };
            }

            self.attraction_points.push(new_point);
        }
    }
}

impl Drawable for SpaceColonization {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.id_trans();

        for node in self.nodes.iter() {
            node.draw(ctx, gl);
        }
        for pt in self.attraction_points.iter() {
            ellipse(
                [1.0, 0.0, 0.0, 0.3],
                [pt.x - 2.0, pt.y - 2.0, 4.0, 4.0],
                transform,
                gl,
            );
        }
    }
}

impl Drawable for TreeNode {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.id_trans();
        if let Some(pt) = self.next {
            let thickness =
                (MAX_THICK - MIN_THICK) * (self.pos.y / ctx.args.window_size[1]) + MIN_THICK;
            line(
                [1.0, 0.0, 1.0, 1.0],
                thickness,
                [self.pos.x, self.pos.y, pt.x, pt.y],
                transform,
                gl,
            );
        }
        ellipse(
            [1.0, 1.0, 0.0, 1.0],
            [self.pos.x - 1.0, self.pos.y - 1.0, 2.0, 2.0],
            transform,
            gl,
        );
    }
}

impl Updatable for SpaceColonization {
    fn update(&mut self, _: &UpdateContext) {
        struct Indices {
            point_index: usize,
            node_index: usize,
        }
        let mut indices = vec![];
        let mut to_remove = vec![];
        for (ind, pt) in self.attraction_points.iter().enumerate() {
            if let Some(node_ind) = self.closest_node(pt) {
                if pt.dist(&self.nodes[node_ind].pos) < KILL_DIST {
                    to_remove.push(ind);
                } else {
                    indices.push(Indices {
                        point_index: ind,
                        node_index: node_ind,
                    });
                }
            }
        }

        let mut new_nodes = vec![];
        for (ind, node) in self.nodes.iter().enumerate() {
            let closest: Vec<&Vec2D> = indices
                .iter()
                .filter(|inds| inds.node_index == ind)
                .map(|ind| &self.attraction_points[ind.point_index])
                .collect();
            if closest.is_empty() {
                continue;
            }
            let n = node.avg_distance(&closest);
            let new_pos = node.pos + DIST_NEXT * n;
            let mut new_node = TreeNode::new(new_pos.x, new_pos.y);
            new_node.next = Some(node.pos);
            new_nodes.push(new_node);
        }

        self.nodes.extend(new_nodes);
        to_remove.sort();
        to_remove.reverse();
        for ind in to_remove {
            self.attraction_points.remove(ind);
        }
    }
}

impl EventHandler for SpaceColonization {
    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.generate_attraction(ctx.window_size[0], ctx.window_size[1]);
    }
}

impl Runnable for SpaceColonization {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }

    fn setup(&mut self, ctx: &SetupContext) {
        self.generate_attraction(ctx.window_width, ctx.window_height);
        self.nodes.push(TreeNode::new(
            ctx.window_width / 2.0,
            ctx.window_height - 2.0,
        ))
    }
}
