use piston::input::UpdateArgs;

pub struct UpdateContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub mouse_pos: [f64; 2],
    pub args: &'a UpdateArgs,
}

impl<'a> UpdateContext<'a> {
    pub fn frame_rate(&self) -> f64 {
        1.0 / self.args.dt
    }
}
pub trait Updatable {
    fn update(&mut self, _: &UpdateContext) {}
}
