use wolf_engine_events::{dynamic::AnyEvent, event_loop::EventLoop};

use crate::WindowContext;

/// Window backend events.
pub mod event {
    use wolf_engine_events::dynamic::Event;

    use crate::{Uuid, WindowSettings};

    pub use crate::context::WindowContextEventSender;

    /// An event used for communication between the [`WindowContext`](crate::WindowContext), and
    /// the [`WindowSystem`](crate::backend::WindowSystem).
    #[derive(Clone, PartialEq, Debug)]
    pub enum WindowContextEvent {
        /// Emitted when the front-end is requesting a new name for a window.
        ///
        /// # Portability
        ///
        /// Handling this event is optional, because not all platforms / window systems support it.
        WindowRenameRequested(Uuid, String),

        /// Emitted when a window should be redrawn.
        ///
        /// # Portability
        ///
        /// Handling this event is optional, because not all platforms / window systems support it.
        WindowRedrawRequested(Uuid),

        /// Emitted when a window is resized.
        WindowResized(Uuid, u32, u32),

        /// Emitted when a new window is created.
        WindowCreated(Uuid, WindowSettings),

        /// Emitted when all copies of a window have been dropped, and it should be closed.
        WindowClosed(Uuid),

        /// Emitted when the main-loop should exit.
        Exited,
    }

    impl Event for WindowContextEvent {}
}

/// A window system implementation.
pub trait WindowSystem: EventLoop<AnyEvent> {
    fn context(&self) -> WindowContext;
}
