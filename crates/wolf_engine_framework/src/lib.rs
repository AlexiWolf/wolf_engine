pub fn init() -> EngineBuilder {
    EngineBuilder
}

pub struct Engine;

impl Engine {
    pub fn run<G: Game>(self, game: G) {
        todo!()
    }
}

pub struct EngineBuilder;

impl EngineBuilder {
    pub fn build(self) -> Result<Engine, ()> {
        todo!()
    }
}

pub trait Game {
    fn setup(&mut self, context: &mut Context) {}
}

pub struct Context {}
