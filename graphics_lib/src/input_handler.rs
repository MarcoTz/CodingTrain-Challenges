use piston::ButtonArgs;

pub trait InputHandler {
    fn handle(&mut self, _: &ButtonArgs) {}
}
