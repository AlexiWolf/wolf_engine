//! Provides a simple, high-level window system.
//!
//! # Initializing the Window System
//!
//! Initialize the window system by calling the [`init()`] function.
//!
//! ```no_run
//! let window_context = wolf_engine_window::init().build().unwrap();
//! ```
//!
//! Once you've created the [`EventLoop`](event::EventLoop), you can call its
//! [`run()`](event::EventLoop::run()) method to start the window system with the provided
//! event-handling function.  
//!
//! # Handling Events, and Creating Window
//!
//! ```no_run
//! # use wolf_engine_window::{
//! #   Window,
//! #   WindowSettings,
//! #   event::{Event, WindowEvent},
//! # };
//! #
//! # let (event_loop, context) = wolf_engine_window::init().build().unwrap();
//! #
//! let mut window: Option<Window> = None;
//! event_loop.run(|event| match event {
//!     // The main-loop has started.
//!     // Do intial setup, like creating windows, loading assets, ext. here.
//!     Event::Started => {
//!         println!("Hello, world!");
//!         context.create_window(
//!             WindowSettings::default()
//!                 .with_title("Example Window")
//!                 .with_size((800, 600))
//!             );
//!     }
//!     // All events have been processed.
//!     Event::EventsCleared => {
//!         if let Some(window) = window.as_ref() {
//!             // Start the next frame.
//!             window.redraw();
//!         }
//!     }
//!     // Window-specific events.
//!     Event::WindowEvent(window_id, event) => match event {
//!         // A window has either been created, or has failed for some reason.
//!         // You can unwrap the result, and store the window.
//!         // This is where you're going to want to set up your renderer, also.
//!         WindowEvent::Created(window_result) => window = Some(window_result.unwrap()),
//!         // A window should be redrawn.
//!         WindowEvent::RedrawRequested => {
//!             // Render code goes here!
//!         },
//!         // A window has / should close.
//!         WindowEvent::Closed => {
//!             context.exit(); // Stop the event loop.
//!         }
//!         _ => (),
//!     }
//!     // The main-loop will stop.
//!     Event::Exited => println!("Goodbye, World!"),
//!     _ => (),
//! });
//! ```
//!
//! # Drawing on the Window
//!
//! This crate doesn't provide its own rendering functions.  Instead, it implements
//! [`raw_window_handle`] traits in order for compatibility with external rendering libraries.

pub mod backend;
mod context;
pub use context::*;
mod window;
pub use window::*;

pub mod event;
pub use uuid::Uuid;

/// Error-types used by the window system.
pub mod error;
/// Re-exports supported [`raw_window_handle`](crate::raw_window_handle::rwh_06) versions.
pub mod raw_window_handle;

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
