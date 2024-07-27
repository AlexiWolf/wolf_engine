//! Provides a high-level game framework.

use std::marker::PhantomData;

use wolf_engine_events::{
    mpsc::{event_queue, MpscEventReceiver, MpscEventSender},
    EventReceiver, EventSender,
};
use wolf_engine_input::Input;
use wolf_engine_window::{Window, WindowContext, WindowEvent};

/// Initialize the [`Engine`].
pub fn init() -> EngineBuilder {
    EngineBuilder
}

/// Run the provided [`Game`].
pub fn run<E: EventHandler>(engine: Engine, game: Game<E>) {
    let mut context = engine.context;
    let mut event_receiver = engine.event_receiver;
    let window_context = engine.window_context;
    let mut unloaded_game = Some(game);
    let mut loaded_game = None;

    window_context.run(|event, window_context| match event {
        WindowEvent::Resumed => {
            let game = std::mem::take(&mut unloaded_game)
                .expect("game should not have been previously loaded");
            let window = window_context.window();
            context.insert_window(window);
            loaded_game = Some(game.setup(&mut context));
        }
        WindowEvent::RedrawRequested => {
            while let Some(event) = event_receiver.next_event() {
                match event {
                    Event::Quit => window_context.window().close(),
                }
            }
            let game = loaded_game
                .as_mut()
                .expect("game should have been loaded on Resume");
            game.render(&mut context);
            game.update(&mut context);
        }
        WindowEvent::Input(input) => {
            let game = loaded_game
                .as_mut()
                .expect("game should have been loaded on Resume");
            game.input(&mut context, input);
        }
        WindowEvent::Resized(width, height) => {
            let game = loaded_game
                .as_mut()
                .expect("game should have been loaded on Resume");
            game.resized(&mut context, (width, height));
        }
        WindowEvent::Closed => {
            let game =
                std::mem::take(&mut loaded_game).expect("game should have been loaded on Resume");
            game.shutdown(&mut context);
        }
        _ => (),
    })
}

pub struct Engine {
    event_receiver: MpscEventReceiver<Event>,
    context: Context,
    window_context: WindowContext,
}

/// Provides a way to configure the engine.
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

/// Type-states used by the [`Game`] struct.
pub mod game_state {
    /// Indicates the [`Game`](crate::Game) has not been loaded yet.
    pub struct Unloaded;

    /// Indicates the [`Game`](crate::Game) has been loaded.
    pub struct Loaded;
}

/// Provides a wrapper around an [`EventHandler`] which implements a program lifetime for the game
/// using type-states.
///
/// This wrapper ensures the following invariants are true:
/// - The [`setup()`](EventHandler::setup()) method is always the first method called.
/// - The [`shutdown()`](EventHandler::shutdown()) is always the last method called.
/// - Each of these methods are only called a single time each through the life of the program.
pub struct Game<E: EventHandler, State = game_state::Unloaded> {
    event_handler: E,
    _state: PhantomData<State>,
}

impl<E: EventHandler> Game<E> {
    /// Create a new game instance.
    pub fn new(event_handler: E) -> Game<E, game_state::Unloaded> {
        Game {
            event_handler,
            _state: PhantomData,
        }
    }
}

impl<E: EventHandler> Game<E, game_state::Unloaded> {
    /// Run one-time setup at the beginning of the program's lifecycle.
    ///
    /// This method must be called before any other methods can be called.
    ///
    /// Calling this method will consume the [`Unloaded`](game_state::Unloaded) game, and return
    /// it in the [`Loaded`](game_state::Loaded) state.
    pub fn setup(mut self, context: &mut Context) -> Game<E, game_state::Loaded> {
        self.event_handler.setup(context);
        Game::<E, game_state::Loaded> {
            event_handler: self.event_handler,
            _state: PhantomData,
        }
    }
}

impl<E: EventHandler> Game<E, game_state::Loaded> {
    /// Handle an input event.
    pub fn input(&mut self, context: &mut Context, input: Input) {
        self.event_handler.input(context, input);
    }

    /// Handle a resized event.
    pub fn resized(&mut self, context: &mut Context, new_size: (u32, u32)) {
        self.event_handler.resized(context, new_size);
    }

    /// Update the game's state.
    pub fn update(&mut self, context: &mut Context) {
        self.event_handler.update(context);
    }

    /// Render the game to the screen.
    pub fn render(&mut self, context: &mut Context) {
        self.event_handler.render(context);
    }

    /// Run one-time teardown at the end of the program's lifecycle.
    ///
    /// Calling this method will consume the game, and drop it.
    pub fn shutdown(mut self, context: &mut Context) {
        self.event_handler.shutdown(context);
    }
}

/// A general-purpose event-handler.
#[cfg_attr(test, mockall::automock)]
pub trait EventHandler {
    /// Run one-time setup when the engine starts.
    fn setup(&mut self, context: &mut Context) {
        let _ = context;
    }

    /// Run one-time teardown when the engine quits.
    fn shutdown(&mut self, context: &mut Context) {
        let _ = context;
    }

    /// Handle input events.
    fn input(&mut self, context: &mut Context, input: Input) {
        let _ = context;
        let _ = input;
    }

    /// Handle [`Resized`](wolf_engine_window::WindowEvent::Resized) events.
    fn resized(&mut self, context: &mut Context, new_size: (u32, u32)) {
        let _ = context;
        let _ = new_size;
    }

    /// Run, and update state.
    fn update(&mut self, context: &mut Context);

    /// Render to the screen.
    fn render(&mut self, context: &mut Context);
}

#[non_exhaustive]
enum Event {
    Quit,
}

/// Provides a container for engine-state.
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
