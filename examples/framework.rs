use wolf_engine::framework::*;

pub fn main() {
    let engine = wolf_engine::framework::init().build().unwrap();
    engine.run(MyGame::new())
}

pub struct MyGame {}

impl MyGame {
    pub fn new() -> Self {
        Self {}
    }
}

impl Game for MyGame {
    fn setup(&mut self, context: &mut Context) {}
}
