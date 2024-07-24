pub trait Game {
    fn setup(&mut self, context: &Context) {}
}

pub struct Context {}
