#[cfg(feature = "events")]
pub use wolf_engine_events as events;

#[cfg(feature = "input")]
pub use wolf_engine_input as input;

#[cfg(feature = "window")]
pub use wolf_engine_window as window;

pub mod prelude {
    #[cfg(feature = "events")]
    pub use crate::events::EventReceiver;
}
