use super::{EDGE_THICK, VERT_SIZE};
use graphics::{colors::Rgba, ellipse, line, Drawable, DrawingContext, Graphics};
use math::vec2d::Vec2D;

pub struct Vertex {
    pos: Vec2D,
    color: Rgba,
}

impl Vertex {
    pub fn new(x: f64, y: f64) -> Vertex {
        Vertex {
            pos: Vec2D::new(x, y),
            color: Rgba::random(),
        }
    }
}

pub struct Graph {
    vertices: Vec<Vertex>,
    active_color: Rgba,
    pub active_path: Vec<usize>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: vec![],
            active_path: vec![],
            active_color: Rgba::random(),
        }
    }

    pub fn add_vertex(&mut self, x: f64, y: f64) {
        let new_vert = Vertex::new(x, y);
        self.vertices.push(new_vert);
    }

    pub fn weight(&self, from: usize, to: usize) -> f64 {
        let from_vert = if let Some(vert) = self.vertices.get(from) {
            vert
        } else {
            return f64::INFINITY;
        };
        let to_vert = if let Some(vert) = self.vertices.get(to) {
            vert
        } else {
            return f64::INFINITY;
        };

        from_vert.pos.dist(&to_vert.pos)
    }

    pub fn weight_path(&self, path: &[usize]) -> f64 {
        let mut weight = 0.0;
        for pair in path.windows(2) {
            weight += self.weight(pair[0], pair[1]);
        }
        weight
    }
}

impl Drawable for Vertex {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        ellipse(
            self.color.into(),
            [
                self.pos.x - VERT_SIZE / 2.0,
                self.pos.y - VERT_SIZE / 2.0,
                VERT_SIZE,
                VERT_SIZE,
            ],
            transform,
            gl,
        );
    }
}

impl Drawable for Graph {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        for vert in self.vertices.iter() {
            vert.draw(ctx, gl);
        }
        for pair in self.active_path.windows(2) {
            let vert1 = &self.vertices[pair[0]];
            let vert2 = &self.vertices[pair[1]];
            line(
                self.active_color.into(),
                EDGE_THICK,
                [vert1.pos.x, vert1.pos.y, vert2.pos.x, vert2.pos.y],
                transform,
                gl,
            );
        }
    }
}
