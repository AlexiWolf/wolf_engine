use wolf_engine_events::{dynamic::AnyEvent, event_loop::EventLoop};

use crate::WindowContext;

pub trait WindowSystem: EventLoop<AnyEvent> {
    fn context(&self) -> WindowContext;
}
