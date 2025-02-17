use super::vec2d::Vec2D;
use std::ops::Mul;

pub struct Mat22 {
    a00: f64,
    a01: f64,
    a10: f64,
    a11: f64,
}

impl Mat22 {
    pub fn new(rows: [[f64; 2]; 2]) -> Mat22 {
        Mat22 {
            a00: rows[0][0],
            a01: rows[0][1],
            a10: rows[1][0],
            a11: rows[1][1],
        }
    }

    pub fn transpose(self) -> Mat22 {
        Mat22::new([[self.a00, self.a10], [self.a01, self.a11]])
    }

    pub fn det(&self) -> f64 {
        self.a00 * self.a11 - self.a01 * self.a10
    }

    pub fn inv(self) -> Option<Mat22> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }
        Some(Mat22::new([[self.a11, -self.a01], [-self.a10, self.a00]]) * (1.0 / det))
    }
}

impl From<[[f64; 2]; 2]> for Mat22 {
    fn from(rows: [[f64; 2]; 2]) -> Mat22 {
        Mat22::new(rows)
    }
}

impl From<[Vec2D; 2]> for Mat22 {
    fn from(col_vecs: [Vec2D; 2]) -> Mat22 {
        Mat22::new([
            [col_vecs[0].x, col_vecs[1].x],
            [col_vecs[0].y, col_vecs[1].y],
        ])
    }
}

impl Mul<Vec2D> for Mat22 {
    type Output = Vec2D;
    fn mul(self, other: Vec2D) -> Vec2D {
        let new_x = self.a00 * other.x + self.a01 * other.y;
        let new_y = self.a10 * other.x + self.a11 * other.y;
        Vec2D::new(new_x, new_y)
    }
}

impl Mul<f64> for Mat22 {
    type Output = Mat22;
    fn mul(self, other: f64) -> Mat22 {
        Mat22::new([
            [other * self.a00, other * self.a01],
            [other * self.a10, other * self.a01],
        ])
    }
}

impl Mul<Mat22> for f64 {
    type Output = Mat22;
    fn mul(self, other: Mat22) -> Mat22 {
        Mat22::new([
            [self * other.a00, self * other.a01],
            [self * other.a10, self * other.a01],
        ])
    }
}

impl Mul<Mat22> for Vec2D {
    type Output = Vec2D;
    fn mul(self, other: Mat22) -> Vec2D {
        other.transpose() * self
    }
}
