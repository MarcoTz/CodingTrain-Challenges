use crate::{colors::Rgba, line, Drawable, DrawingContext, Graphics, Transformed};
use math::vec2d::Vec2D;

pub struct PolyOutline {
    center: Vec2D,
    verts: Vec<Vec2D>,
    line_thickness: f64,
    color: Rgba,
}

impl PolyOutline {
    pub fn new(center: Vec2D, verts: Vec<Vec2D>, color: Rgba) -> PolyOutline {
        PolyOutline {
            center,
            verts,
            line_thickness: 1.0,
            color,
        }
    }

    pub fn vertices(&self) -> Vec<Vec2D> {
        self.verts.iter().copied().collect()
    }

    pub fn center(&self) -> Vec2D {
        self.center
    }

    pub fn color(&self) -> Rgba {
        self.color
    }

    pub fn set_vertices(&mut self, verts: Vec<Vec2D>) {
        self.verts = verts;
    }

    pub fn set_center(&mut self, new_center: Vec2D) {
        self.center = new_center;
    }

    pub fn set_line(&mut self, thickness: f64) {
        self.line_thickness = thickness;
    }

    pub fn diameter(&self) -> f64 {
        let mut max_dist = 0.0;
        for vert1 in self.verts.iter() {
            for vert2 in self.verts.iter() {
                let dist = vert1.dist(&vert2);
                if dist > max_dist {
                    max_dist = dist
                }
            }
        }
        max_dist
    }

    pub fn edges(&self) -> Vec<[Vec2D; 2]> {
        let mut edges = vec![];
        for edg in self.verts.windows(2) {
            edges.push([edg[0], edg[1]]);
        }
        edges.push([self.verts[self.verts.len() - 1], self.verts[0]]);
        edges
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

    pub fn collides(&self, other: &PolyOutline) -> bool {
        let normals1 = self.normals();
        let poly1_proj1 = normals1.iter().map(|n| self.proj_verts(*n));
        let poly2_proj1 = normals1.iter().map(|n| other.proj_verts(*n));

        let normals2 = other.normals();
        let poly1_proj2 = normals2.iter().map(|n| self.proj_verts(*n));
        let poly2_proj2 = normals2.iter().map(|n| other.proj_verts(*n));

        let overlap = |(p1, p2): ([f64; 2], [f64; 2])| {
            (p1[1] > p2[0] && p1[1] < p2[1]) || (p2[1] > p1[0] && p2[1] < p1[1])
        };

        poly1_proj1.clone().zip(poly2_proj1.clone()).all(overlap)
            && poly1_proj2.zip(poly2_proj2).all(overlap)
    }
}

impl Drawable for PolyOutline {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.context.transform.trans(self.center.x, self.center.y);
        let mut verts = self.verts.clone();
        verts.push(self.verts[0]);
        for end_points in verts.windows(2) {
            line(
                self.color.into(),
                self.line_thickness,
                [
                    end_points[0].x,
                    end_points[0].y,
                    end_points[1].x,
                    end_points[1].y,
                ],
                transform,
                gl,
            );
        }
    }
}
