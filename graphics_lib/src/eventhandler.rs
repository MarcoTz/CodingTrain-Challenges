use piston::{ButtonArgs, ResizeArgs};

pub struct InputContext<'a> {
    pub window_height: f64,
    pub window_width: f64,
    pub mouse_pos: [f64; 2],
    pub args: &'a ButtonArgs,
}
pub trait EventHandler {
    fn handle_input(&mut self, _: &InputContext) {}
    fn handle_resize(&mut self, _: &ResizeArgs) {}
}
