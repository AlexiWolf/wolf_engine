use pixels::Pixels;
use wolf_engine::framework::*;

pub fn main() {
    let engine = wolf_engine::framework::init().build().unwrap();
    wolf_engine::framework::run(engine, MyGame::new())
}

pub struct MyGame {
    pixels: Option<Pixels>,
}

impl MyGame {
    pub fn new() -> Self {
        Self { pixels: None }
    }
}

impl Game for MyGame {
    fn setup(&mut self, context: &mut Context) {
        let window = context.window();
    }
    fn shutdown(&mut self, context: &mut Context) {}
    fn update(&mut self, context: &mut Context) {}
    fn render(&mut self, context: &mut Context) {}
}
