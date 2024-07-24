pub use wolf_engine::framework::*;

pub fn main() {}

pub struct MyGame;

impl Game for MyGame {
    fn setup(&mut self, context: &mut Context) {}
}
