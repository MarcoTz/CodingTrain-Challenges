pub mod grid;
pub mod mat22;
pub mod vec2d;

pub fn rand_between(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max - min)
}
