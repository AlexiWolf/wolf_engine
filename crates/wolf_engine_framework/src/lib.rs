use wolf_engine_events::{
    mpsc::{event_queue, MpscEventReceiver, MpscEventSender},
    EventReceiver, EventSender,
};
use wolf_engine_window::{Window, WindowContext, WindowEvent};

pub fn init() -> EngineBuilder {
    EngineBuilder
}

pub fn run<G: Game>(engine: Engine, mut game: G) {
    let mut context = engine.context;
    let mut event_receiver = engine.event_receiver;
    let window_context = engine.window_context;

    window_context.run(|event, window_context| match event {
        WindowEvent::Resumed => {
            let window = window_context.window();
            context.insert_window(window);
            game.setup(&mut context);
        }
        WindowEvent::RedrawRequested => {
            while let Some(event) = event_receiver.next_event() {
                match event {
                    Event::Quit => window_context.window().close(),
                }
            }
            game.render(&mut context);
            game.update(&mut context);
        }
        WindowEvent::Closed => game.shutdown(&mut context),
        _ => (),
    })
}

pub struct Engine {
    event_receiver: MpscEventReceiver<Event>,
    context: Context,
    window_context: WindowContext,
}

pub struct EngineBuilder;

impl EngineBuilder {
    pub fn build(self) -> Result<Engine, ()> {
        let window_context = wolf_engine_window::init().build().unwrap();
        Ok(Self::setup_engine(window_context))
    }

    #[cfg(test)]
    #[doc(hidden)]
    pub fn build_any_thread(self) -> Result<Engine, ()> {
        let window_context = wolf_engine_window::init()
            .with_visible(false)
            .build_any_thread()
            .unwrap();
        Ok(Self::setup_engine(window_context))
    }

    fn setup_engine(window_context: WindowContext) -> Engine {
        let (event_sender, event_receiver) = event_queue::<Event>();
        let context = Context::new(event_sender);
        Engine {
            event_receiver,
            context,
            window_context,
        }
    }
}

pub trait Game {
    fn setup(&mut self, context: &mut Context) {}
    fn shutdown(&mut self, context: &mut Context) {}
    fn update(&mut self, context: &mut Context);
    fn render(&mut self, context: &mut Context);
}

#[non_exhaustive]
enum Event {
    Quit,
}

pub struct Context {
    event_sender: MpscEventSender<Event>,
    window: Option<Window>,
}

impl Context {
    fn new(event_sender: MpscEventSender<Event>) -> Self {
        Self {
            event_sender,
            window: None,
        }
    }

    pub fn window(&self) -> Window {
        self.window
            .as_ref()
            .expect("There is no Window yet")
            .clone()
    }

    pub fn insert_window(&mut self, window: Window) {
        self.window = Some(window);
    }

    pub fn quit(&self) {
        self.event_sender.send_event(Event::Quit).unwrap();
    }
}

#[cfg(test)]
mod framework_tests {
    use super::*;

    #[derive(Default)]
    struct CallTestGame {
        update: u32,
    }

    impl Game for CallTestGame {
        fn update(&mut self, context: &mut Context) {
            if self.update < 100 {
                self.update += 1;
            } else {
                context.quit();
            }
        }
        fn render(&mut self, _context: &mut Context) {}
    }

    #[test]
    #[ntest::timeout(100)]
    fn should_run_and_quit() {
        let engine = crate::init().build_any_thread().unwrap();
        crate::run(engine, CallTestGame::default());
    }
}
