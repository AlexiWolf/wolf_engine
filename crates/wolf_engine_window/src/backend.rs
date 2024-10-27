use wolf_engine_events::{dynamic::AnyEvent, event_loop::EventLoop};

use crate::WindowContext;

pub mod event {
    use wolf_engine_events::dynamic::Event;

    use crate::{Uuid, WindowSettings};

    #[derive(Clone, PartialEq, Debug)]
    pub enum WindowContextEvent {
        WindowRenameRequested(Uuid, String),
        WindowRedrawRequested(Uuid),
        WindowResized(Uuid, u32, u32),
        WindowCreated(Uuid, WindowSettings),
        WindowClosed(Uuid),
        Exited,
    }

    impl Event for WindowContextEvent {}
}

pub trait WindowSystem: EventLoop<AnyEvent> {
    fn context(&self) -> WindowContext;
}
