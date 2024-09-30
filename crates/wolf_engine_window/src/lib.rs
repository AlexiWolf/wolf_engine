//! Provides a simple, high-level window system.

mod context;
pub use context::*;
mod window;
use event::Event;
pub use window::*;

pub mod event;
pub use uuid::Uuid;
use wolf_engine_events::{dynamic::AnyEvent, mpsc::MpscEventSender};

/// Error-types used by the window system.
pub mod error;
/// Re-exports supported [`raw_window_handle`](crate::raw_window_handle::rwh_06) versions.
pub mod raw_window_handle;

/// Initialize the window system.
pub fn init(event_sender: MpscEventSender<AnyEvent>) -> WindowContext {
    WindowContext::new(event_sender)
}

pub trait WindowBackend {
    fn context(&self) -> WindowContext;
}

#[cfg(test)]
mod window_system_tests {
    use super::*;

    #[test]
    fn should_set_settings_values() {
        let window_settings = WindowSettings::default()
            .with_title("Custom Test Title")
            .with_size((123, 123))
            .with_resizable(false)
            .with_visible(false)
            .with_fullscreen_mode(FullscreenMode::Borderless);

        assert_eq!(window_settings.title, "Custom Test Title");
        assert_eq!(window_settings.size, (123, 123));
        assert_eq!(window_settings.is_resizable, false);
        assert_eq!(window_settings.is_visible, false);
        assert_eq!(
            window_settings.fullscreen_mode,
            Some(FullscreenMode::Borderless)
        );
    }

    #[test]
    fn should_not_be_fullscreen_by_default() {
        let window_settings = WindowSettings::default();

        assert!(window_settings.fullscreen_mode.is_none());
    }
}
