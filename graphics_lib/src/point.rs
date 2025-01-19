use std::{
    fmt,
    ops::{Add, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn from_polar(r: f64, phi: f64) -> Point {
        let x = r * phi.cos();
        let y = r * phi.sin();
        Point { x, y }
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

    pub fn dist(&self, other: &Point) -> f64 {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
