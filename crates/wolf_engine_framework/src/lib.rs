pub fn init() -> EngineBuilder {
    EngineBuilder
}

pub fn run<G: Game>(engine: Engine, game: G) {
    todo!()
}

pub struct Engine;

pub struct EngineBuilder;

impl EngineBuilder {
    pub fn build(self) -> Result<Engine, ()> {
        todo!()
    }
}

pub trait Game {
    fn setup(&mut self, context: &mut Context) {}
    fn shutdown(&mut self, context: &mut Context) {}
    fn update(&mut self, context: &mut Context);
    fn render(&mut self, context: &mut Context);
}

pub struct Context {}
