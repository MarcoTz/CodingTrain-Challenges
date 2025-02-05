use graphics::{
    ellipse, line, Color, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston::{Button, ButtonState, Key, ResizeArgs};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const NUM_POINTS: usize = 1000;
const DIST_NEXT: f64 = 50.0;
const INF_RADIUS: f64 = 15.0 * DIST_NEXT;
const KILL_DIST: f64 = 20.0;

const MAX_THICK: f64 = 3.0;
const MIN_THICK: f64 = 1.0;
const NODE_COLOR: Color = [1.0, 0.5, 0.2, 1.0];
const ENVELOPE_RADIUS: f64 = 400.0;

const MAX_LEAVES: usize = 20;
const LEAF_SIZE: f64 = 5.0;
const LEAF_RATE: f64 = 0.5;

struct TreeNode {
    pos: Vec2D,
    next: Vec<Vec2D>,
    finished: bool,
    leaves: Vec<(Vec2D, f64)>,
}

impl TreeNode {
    pub fn new(x: f64, y: f64) -> TreeNode {
        TreeNode {
            pos: Vec2D::new(x, y),
            next: vec![],
            finished: false,
            leaves: vec![],
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

    fn generate_leaves(&mut self, last: Vec2D) {
        for _ in 0..MAX_LEAVES {
            if rand::random::<f64>() < LEAF_RATE {
                continue;
            }
            let dist = rand::random::<f64>();
            let leaf_pt = ((1.0 - dist) * self.pos) + (dist * last);
            let leaf_size = rand::random::<f64>() * LEAF_SIZE;
            self.leaves.push((leaf_pt, leaf_size));
        }
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
        let envelope_center = Vec2D {
            x: window_width / 2.0,
            y: window_height / 3.0,
        };
        for _ in 0..NUM_POINTS {
            let mut new_point = Vec2D {
                x: rand::random::<f64>() * window_width,
                y: rand::random::<f64>() * window_height,
            };
            while new_point.dist(&envelope_center) > ENVELOPE_RADIUS
                || self.attraction_points.contains(&new_point)
            {
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
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for node in self.nodes.iter() {
            node.draw(ctx, gl);
        }
    }
}

impl Drawable for TreeNode {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for next in self.next.iter() {
            let thickness =
                (MAX_THICK - MIN_THICK) * (self.pos.y / ctx.args.window_size[1]) + MIN_THICK;
            line(
                NODE_COLOR,
                thickness,
                [self.pos.x, self.pos.y, next.x, next.y],
                transform,
                gl,
            );
            for leaf in self.leaves.iter() {
                ellipse(
                    [0.0, 1.0, 0.0, 1.0],
                    [
                        leaf.0.x - leaf.1,
                        leaf.0.y - leaf.1,
                        leaf.1 * 2.0,
                        leaf.1 * 2.0,
                    ],
                    transform,
                    gl,
                );
            }
        }

        if self.next.is_empty() {
            ellipse(
                [1.0, 0.0, 0.8, 1.0],
                [self.pos.x - 8.0, self.pos.y - 8.0, 16.0, 16.0],
                transform,
                gl,
            );
        }
    }
}

impl Updatable for SpaceColonization {
    fn update(&mut self, ctx: &UpdateContext) {
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
        let existing_pos: Vec<Vec2D> = self.nodes.iter().map(|node| node.pos).collect();

        for (ind, node) in self.nodes.iter_mut().enumerate() {
            if node.finished {
                continue;
            }
            let closest: Vec<&Vec2D> = indices
                .iter()
                .filter(|inds| inds.node_index == ind)
                .map(|ind| &self.attraction_points[ind.point_index])
                .collect();
            if closest.is_empty() {
                node.finished = true;
                continue;
            }
            let n = node.avg_distance(&closest);
            let new_pos = node.pos + DIST_NEXT * n;
            if existing_pos.contains(&new_pos) {
                continue;
            }
            let new_node = TreeNode::new(new_pos.x, new_pos.y);
            node.next.push(new_pos);
            if new_pos.dist(&Vec2D {
                x: ctx.window_width / 2.0,
                y: ctx.window_height / 2.0,
            }) < INF_RADIUS
            {
                node.generate_leaves(new_pos);
            }
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

    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state == ButtonState::Release && ctx.args.button == Button::Keyboard(Key::Space)
        {
            self.nodes.clear();
            self.generate_attraction(ctx.window_width, ctx.window_height);
            self.nodes.push(TreeNode::new(
                ctx.window_width / 2.0,
                ctx.window_height - 2.0,
            ))
        }
    }
}

impl Runnable for SpaceColonization {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Space Colonization".to_owned(),
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
