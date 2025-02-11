use super::rgba::Rgba;
use std::fmt;

/// HSV color
/// 0<=h<=360
/// 0<=s,v<=1
#[derive(Debug, Clone, Copy)]
pub struct Hsv {
    pub(crate) h: f64,
    pub(crate) s: f64,
    pub(crate) v: f64,
}

impl Hsv {
    pub fn new(h: f64, s: f64, v: f64) -> Option<Hsv> {
        if h < 0.0 || h > 360.0 || s < 0.0 || s > 1.0 || v < 0.0 || v > 1.0 {
            return None;
        }
        Some(Hsv { h, s, v })
    }

    pub fn from_rgba(rgba: Rgba) -> Hsv {
        let r_prime = rgba.r as f64 / 255.0;
        let g_prime = rgba.g as f64 / 255.0;
        let b_prime = rgba.b as f64 / 255.0;
        let c_max = r_prime.max(g_prime).max(b_prime);
        let c_min = r_prime.min(g_prime).min(b_prime);
        let delta = c_max - c_min;

        let mut h = 60.0
            * if c_max == r_prime {
                (g_prime - b_prime) / delta
            } else if c_max == g_prime {
                (b_prime - r_prime) / delta
            } else if c_max == b_prime {
                (r_prime - g_prime) / delta
            } else {
                panic!("Invalid Rgb values")
            };

        if h < 0.0 {
            h += 360.0;
        }

        let v = c_max;
        let s = delta / c_max;
        Hsv { h, s, v }
    }

    pub fn increase_hue(&mut self, amount: f64) {
        self.h += amount;
        if self.h > 360.0 {
            self.h -= 360.0
        }
    }
}

impl From<Rgba> for Hsv {
    fn from(rgba: Rgba) -> Hsv {
        Hsv::from_rgba(rgba)
    }
}

impl fmt::Display for Hsv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.h, self.s, self.v)
    }
}
