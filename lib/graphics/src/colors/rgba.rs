use super::hsv::Hsv;
use std::fmt;

fn hex2num(hex: char) -> Option<u8> {
    match hex.to_ascii_uppercase() {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'A' => Some(10),
        'B' => Some(11),
        'C' => Some(12),
        'D' => Some(13),
        'E' => Some(14),
        'F' => Some(15),
        _ => None,
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

pub const WHITE: Rgba = Rgba {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub const BLACK: Rgba = Rgba {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const RED: Rgba = Rgba {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
pub const GREEN: Rgba = Rgba {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
pub const BLUE: Rgba = Rgba {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

pub const YELLOW: Rgba = Rgba {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }

    pub fn random() -> Rgba {
        Rgba {
            r: rand::random(),
            g: rand::random(),
            b: rand::random(),
            a: 255,
        }
    }

    pub fn from_hex(hex: &str) -> Option<Rgba> {
        let mut chars = hex.chars();

        if hex.starts_with('#') {
            chars.next();
        }

        let r_fst = hex2num(chars.next()?)?;
        let r_snd = hex2num(chars.next()?)?;
        let g_fst = hex2num(chars.next()?)?;
        let g_snd = hex2num(chars.next()?)?;
        let b_fst = hex2num(chars.next()?)?;
        let b_snd = hex2num(chars.next()?)?;

        if chars.next().is_some() {
            return None;
        }

        Some(Rgba {
            r: r_fst * 16 + r_snd,
            g: g_fst * 16 + g_snd,
            b: b_fst * 16 + b_snd,
            a: 255,
        })
    }

    pub fn from_hsv(hsv: Hsv) -> Rgba {
        if hsv.s == 0.0 {
            return Rgba {
                r: (hsv.v * 255.0).round() as u8,
                g: (hsv.v * 255.0).round() as u8,
                b: (hsv.v * 255.0).round() as u8,
                a: 255,
            };
        }
        let mut hh = hsv.h;
        if hh >= 360.0 {
            hh = 0.0;
        }
        hh /= 60.0;
        let i = hh as u64;
        let ff = hh - (i as f64);
        let p = hsv.v * (1.0 - hsv.s);
        let q = hsv.v * (1.0 - (hsv.s * ff));
        let t = hsv.v * (1.0 - (hsv.s * (1.0 - ff)));

        match i {
            0 => Rgba {
                r: (hsv.v * 255.0).round() as u8,
                g: (t * 255.0).round() as u8,
                b: (p * 255.0).round() as u8,
                a: 255,
            },
            1 => Rgba {
                r: (q * 255.0).round() as u8,
                g: (hsv.v * 255.0).round() as u8,
                b: (p * 255.0).round() as u8,
                a: 255,
            },
            2 => Rgba {
                r: (p * 255.0).round() as u8,
                g: (hsv.v * 255.0).round() as u8,
                b: (t * 255.0).round() as u8,
                a: 255,
            },
            3 => Rgba {
                r: (p * 255.0).round() as u8,
                g: (q * 255.0).round() as u8,
                b: (hsv.v * 255.0).round() as u8,
                a: 255,
            },
            4 => Rgba {
                r: (t * 255.0).round() as u8,
                g: (p * 255.0).round() as u8,
                b: (hsv.v * 255.0).round() as u8,
                a: 255,
            },
            5 => Rgba {
                r: (hsv.v * 255.0).round() as u8,
                g: (p * 255.0).round() as u8,
                b: (q * 255.0).round() as u8,
                a: 255,
            },
            _ => panic!("Invalid value for hue"),
        }
    }

    pub fn lerp(self, other: Rgba, t: f64) -> Rgba {
        if t < 0.0 {
            return self;
        }
        if t > 1.0 {
            return other;
        }

        Rgba {
            r: ((1.0 - t) * self.r as f64 + t * other.r as f64).round() as u8,
            g: ((1.0 - t) * self.g as f64 + t * other.g as f64).round() as u8,
            b: ((1.0 - t) * self.b as f64 + t * other.b as f64).round() as u8,
            a: ((1.0 - t) * self.a as f64 + t * other.a as f64).round() as u8,
        }
    }

    pub fn with_trans(self, trans: u8) -> Rgba {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.g,
            a: trans,
        }
    }
}

impl Into<[f32; 4]> for Rgba {
    fn into(self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

impl From<[f32; 4]> for Rgba {
    fn from(col: [f32; 4]) -> Rgba {
        let r = (col[0] * 255.0).round() as u8;
        let g = (col[1] * 255.0).round() as u8;
        let b = (col[2] * 255.0).round() as u8;
        let a = (col[3] * 255.0).round() as u8;
        Rgba::new(r, g, b, a)
    }
}

impl From<Hsv> for Rgba {
    fn from(hsv: Hsv) -> Rgba {
        Rgba::from_hsv(hsv)
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}
