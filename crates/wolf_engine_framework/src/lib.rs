pub trait Game {
    fn setup(&mut self, context: &mut Context) {}
}

pub struct Context {}
