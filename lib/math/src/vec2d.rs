use std::f64::consts::PI;
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }

    pub fn from_polar(r: f64, phi: f64) -> Vec2D {
        let x = r * phi.cos();
        let y = r * phi.sin();
        Vec2D { x, y }
    }

    pub fn polar(&self) -> (f64, f64) {
        (self.abs(), self.arg())
    }

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn set_abs(&mut self, r: f64) {
        let arg = self.arg();
        self.x = r * arg.cos();
        self.y = r * arg.sin();
    }

    pub fn arg(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn set_arg(&mut self, arg: f64) {
        let abs = self.abs();
        self.x = abs * arg.cos();
        self.y = abs * arg.sin();
    }

    pub fn dist(&self, other: &Vec2D) -> f64 {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }

    pub fn rand_unit() -> Vec2D {
        let arg = rand::random::<f64>() * 2.0 * PI;
        Vec2D::from_polar(1.0, arg)
    }

    pub fn tangent(&self) -> Vec2D {
        let arg = self.arg() + PI / 2.0;
        Vec2D::from_polar(1.0, arg)
    }

    pub fn dot(self, other: Vec2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2D {
    type Output = Vec2D;
    fn add(self, other: Vec2D) -> Self::Output {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, other: Vec2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, other: Vec2D) -> Self::Output {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, other: Vec2D) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f64> for Vec2D {
    type Output = Vec2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2D> for f64 {
    type Output = Vec2D;
    fn mul(self, rhs: Vec2D) -> Self::Output {
        Vec2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<f64> for Vec2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vec2D {
    type Output = Vec2D;
    fn div(self, rhs: f64) -> Self::Output {
        Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vec2D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2D {
    type Output = Vec2D;
    fn neg(self) -> Self::Output {
        Vec2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
