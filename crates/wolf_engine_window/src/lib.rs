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
//! #   WindowSettings,
//! #   event::{Event, WindowEvent},
//! # };
//! #
//! # let window_context = wolf_engine_window::init().build().unwrap();
//! #
//! let mut window = None;
//! window_context.run(|event, context| match event {
//!     // The main-loop has started.
//!     // Do intial setup, like creating windows, render surfaces, ext. here.
//!     Event::Started => {
//!         println!("Hello, world!");
//!         window = Some(
//!             context.create_window(
//!                 WindowSettings::default()
//!                     .with_title("Example Window")
//!                     .with_size((800, 600)),
//!             )
//!         );
//!     }
//!     // All events have been processed.
//!     Event::EventsCleared => {
//!         // Start the next frame.
//!         window.unwrap().redraw();
//!     }
//!     // Window-specific events.
//!     Event::WindowEvent(window_id, event) => match event {
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

use event::EventLoopBuilder;

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

/// Initialize the window system.
pub fn init() -> EventLoopBuilder {
    EventLoopBuilder::new()
}

#[cfg(test)]
mod window_init_tests {
    use super::*;

    #[cfg(target_os = "linux")]
    use winit::platform::x11::EventLoopBuilderExtX11;

    #[cfg(target_os = "windows")]
    use winit::platform::windows::EventLoopBuilderExtWindows;

    #[test]
    fn should_set_settings_values() {
        let window_settings = WindowSettings::default()
            .with_title("Custom Test Title")
            .with_size((123, 123))
            .with_resizable(false)
            .with_visible(false);

        assert_eq!(window_settings.title, "Custom Test Title");
        assert_eq!(window_settings.size, (123, 123));
        assert_eq!(window_settings.is_resizable, false);
        assert_eq!(window_settings.is_visible, false);
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[test]
    #[ntest::timeout(1000)]
    fn should_run_and_quit() {
        use crate::event::{Event, WinitEventLoop};

        let event_loop = WinitEventLoop::with_user_event()
            .with_any_thread(true)
            .build()
            .unwrap();
        let context = crate::init().build_with_event_loop(event_loop);

        let mut has_quit = false;

        context.run(|event, context| match event {
            Event::Started => {
                let _window = context
                    .create_window(WindowSettings::default().with_visible(false))
                    .expect("window creation succeeded");
                context.exit();
            }
            Event::Exited => *&mut has_quit = true,
            _ => (),
        });

        assert!(
            has_quit,
            "The window system has not quit, or did not run properly."
        );
    }
}
