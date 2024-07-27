use std::marker::PhantomData;

use wolf_engine_events::{
    mpsc::{event_queue, MpscEventReceiver, MpscEventSender},
    EventReceiver, EventSender,
};
use wolf_engine_input::Input;
use wolf_engine_window::{Window, WindowContext, WindowEvent};

pub fn init() -> EngineBuilder {
    EngineBuilder
}

pub fn run<E: EventHandler>(engine: Engine, mut game: Game<E>) {
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
        WindowEvent::Input(input) => game.input(&mut context, input),
        WindowEvent::Resized(width, height) => game.resized(&mut context, (width, height)),
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

pub mod game_state {
    pub struct Unloaded;
    pub struct Loaded;
}

pub struct Game<E: EventHandler, State = game_state::Unloaded> {
    event_handler: E,
    _state: PhantomData<State>,
}

impl<E: EventHandler> Game<E> {
    pub fn new(event_handler: E) -> Game<E, game_state::Unloaded> {
        Game {
            event_handler,
            _state: PhantomData,
        }
    }
}

impl<E: EventHandler> Game<E, game_state::Unloaded> {
    pub fn setup(mut self, context: &mut Context) -> Game<E, game_state::Loaded> {
        self.event_handler.setup(context);
        Game::<E, game_state::Loaded> {
            event_handler: self.event_handler,
            _state: PhantomData,
        }
    }
}

impl<E: EventHandler> Game<E, game_state::Loaded> {
    pub fn input(&mut self, context: &mut Context, input: Input) {
        self.event_handler.input(context, input);
    }

    pub fn resized(&mut self, context: &mut Context, new_size: (u32, u32)) {
        self.event_handler.resized(context, new_size);
    }

    pub fn update(&mut self, context: &mut Context) {
        self.event_handler.update(context);
    }

    pub fn render(&mut self, context: &mut Context) {
        self.event_handler.render(context);
    }

    pub fn shutdown(mut self, context: &mut Context) {
        self.event_handler.shutdown(context);
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait EventHandler {
    fn setup(&mut self, context: &mut Context) {
        let _ = context;
    }
    fn shutdown(&mut self, context: &mut Context) {
        let _ = context;
    }
    fn input(&mut self, context: &mut Context, input: Input) {
        let _ = context;
        let _ = input;
    }
    fn resized(&mut self, context: &mut Context, new_size: (u32, u32)) {
        let _ = context;
        let _ = new_size;
    }

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
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    #[ntest::timeout(100)]
    fn should_run_and_quit() {
        let updates = Arc::new(Mutex::new(0));
        let mut event_handler = MockEventHandler::new();

        event_handler.expect_setup().once().return_const(());
        event_handler.expect_shutdown().once().return_const(());
        event_handler
            .expect_update()
            // The engine tends to respond to shutdowns a few frames late, and I don't think this
            // is a problem.
            // This range allows a bit of wiggle-room in how exactly how many frames it takes.
            .times(100..110)
            .returning(move |context| {
                let mut updates = updates.lock().unwrap();
                if *updates < 100 {
                    *updates += 1;
                } else {
                    context.quit()
                }
            });
        event_handler.expect_render().times(1..).return_const(());

        let game = Game::new(event_handler);
        let engine = crate::init().build_any_thread().unwrap();
        crate::run(engine, game);
    }
}
