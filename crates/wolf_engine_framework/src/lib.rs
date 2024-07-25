use wolf_engine_events::mpsc::{event_queue, MpscEventReceiver, MpscEventSender};
use wolf_engine_window::WindowContext;

pub fn init() -> EngineBuilder {
    EngineBuilder
}

pub fn run<G: Game>(engine: Engine, game: G) {
    todo!()
}

pub struct Engine {
    event_receiver: MpscEventReceiver<Event>,
    context: Context,
    window_context: WindowContext,
}

pub struct EngineBuilder;

impl EngineBuilder {
    pub fn build(self) -> Result<Engine, ()> {
        let (event_sender, event_receiver) = event_queue::<Event>();
        let context = Context::new(event_sender);
        let window_context = wolf_engine_window::init().build().unwrap();
        let engine = Engine {
            event_receiver,
            context,
            window_context,
        };
        Ok(engine)
    }
}

pub trait Game {
    fn setup(&mut self, context: &mut Context) {}
    fn shutdown(&mut self, context: &mut Context) {}
    fn update(&mut self, context: &mut Context);
    fn render(&mut self, context: &mut Context);
}

struct EventLoop {}

pub struct Context {}

impl Context {
    pub fn quit(&self) {}
}

#[cfg(test)]
mod framework_tests {
    use super::*;

    #[derive(Default)]
    struct CallTestGame {
        setup: u32,
        shutdown: u32,
        update: u32,
        render: u32,
    }

    impl Game for CallTestGame {
        fn setup(&mut self, _context: &mut Context) {
            self.setup += 1;
        }
        fn shutdown(&mut self, _context: &mut Context) {
            self.shutdown += 1;
        }
        fn update(&mut self, context: &mut Context) {
            if self.update < 100 {
                self.update += 1;
            } else {
                context.quit();
            }
        }
        fn render(&mut self, _context: &mut Context) {
            self.render += 1;
        }
    }

    #[test]
    #[ntest::timeout(100)]
    fn should_follow_method_call_expectations() {
        let engine = crate::init().build().unwrap();
        crate::run(engine, CallTestGame::default());
    }
}
