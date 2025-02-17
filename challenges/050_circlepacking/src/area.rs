use graphics::{line, Drawable, DrawingContext, Graphics, Transformed};
use math::vec2d::Vec2D;

pub struct Area {
    center: Vec2D,
    verts: Vec<Vec2D>,
}

impl Area {
    pub fn new(center: Vec2D, verts: Vec<Vec2D>) -> Area {
        Area { center, verts }
    }

    pub fn set_center(&mut self, center: Vec2D) {
        self.center = center;
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vec2D>) {
        self.verts = vertices;
    }

    pub fn edges(&self) -> Vec<[Vec2D; 2]> {
        let mut verts = self.verts.clone();
        verts.push(self.verts[0]);
        verts.windows(2).map(|edg| [edg[0], edg[1]]).collect()
    }

    pub fn normals(&self) -> Vec<Vec2D> {
        let mut normals = vec![];
        for edge in self.edges() {
            let mut norm = (edge[0] - edge[1]).tangent();
            norm.set_abs(1.0);
            normals.push(norm);
        }
        normals
    }

    pub fn proj_verts(&self, onto: Vec2D) -> [f64; 2] {
        let proj = self
            .verts
            .iter()
            .map(|v| self.center + *v)
            .map(|v| v.dot(onto));
        let proj_min = proj
            .clone()
            .min_by(|f1, f2| f1.partial_cmp(f2).unwrap())
            .unwrap();
        let proj_max = proj.max_by(|f1, f2| f1.partial_cmp(f2).unwrap()).unwrap();
        [proj_min, proj_max]
    }

    pub fn inside(&self, other: &Vec2D, r: f64) -> bool {
        let normals1 = self.normals();
        let poly1_proj1 = normals1.iter().map(|n| self.proj_verts(*n));
        let poly2_proj1 = normals1.iter().map(|n| other.dot(*n));

        let overlap = |(p1, p2): ([f64; 2], f64)| {
            (p1[1] > p2 - r && p1[1] < p2 + r) || (p2 + r > p1[0] && p2 + r < p1[1])
        };

        poly1_proj1.clone().zip(poly2_proj1.clone()).all(overlap)
    }
}

impl Drawable for Area {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.context.transform.trans(self.center.x, self.center.y);

        for edg in self.edges() {
            line(
                [1.0, 1.0, 1.0, 1.0],
                1.0,
                [edg[0].x, edg[0].y, edg[1].x, edg[1].y],
                transform,
                gl,
            );
            let mid = 0.5 * edg[0] + 0.5 * edg[1];
            let mut norm = (edg[0] - edg[1]).tangent();
            norm.set_abs(100.0);
            line(
                [1.0, 1.0, 1.0, 1.0],
                1.0,
                [mid.x, mid.y, mid.x + norm.x, mid.y + norm.y],
                transform,
                gl,
            );
        }
    }
}
