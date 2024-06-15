pub mod backend;
pub mod raw_window_handle;

mod context;
mod error;
mod event;
mod settings;
mod window;

use backend::winit::WinitBackend;
pub use context::*;
pub use error::*;
pub use event::*;
pub use settings::*;
pub use uuid::Uuid;
pub use window::*;

pub type WindowSystem = (WindowEventQueue, WindowContext);

pub fn init() -> Result<WindowSystem, &'static str> {
    WinitBackend::init()
}

#[cfg(test)]
mod window_system_tests {
    use std::{
        collections::VecDeque,
        sync::{Arc, RwLock},
        thread,
    };

    use uuid::Uuid;
    use wolf_engine_events::{
        mpsc::{self, MpscEventSender},
        EventReceiver, EventSender,
    };

    use crate::{error::WindowError, event::WindowEvent};

    use self::backend::{WindowBackend, WindowTrait};

    use super::*;

    #[derive(Clone)]
    pub struct TestWindowBackend {
        event_sender: Arc<RwLock<Option<MpscEventSender<WindowEvent>>>>,
        pub buffered_events: Arc<RwLock<VecDeque<WindowEvent>>>,
    }

    impl TestWindowBackend {
        fn init() -> (WindowSystem, Self) {
            let (event_sender, event_receiver) = mpsc::event_queue();
            let adapter = Self {
                event_sender: Arc::new(RwLock::new(Some(event_sender))),
                buffered_events: Arc::new(RwLock::new(VecDeque::new())),
            };
            let context = WindowContext::new(Box::new(adapter.clone()));
            let event_queue = WindowEventQueue::new(&context, event_receiver);
            ((event_queue, context), adapter)
        }

        pub fn buffer_event(&self, event: WindowEvent) {
            self.buffered_events.write().unwrap().push_back(event);
        }
    }

    impl WindowBackend for TestWindowBackend {
        fn pump_events(&self) {
            while let Some(event) = self.buffered_events.write().unwrap().pop_front() {
                let guard = self.event_sender.read().unwrap();
                let event_queue = guard.as_ref().unwrap();
                event_queue.send_event(event).unwrap();
            }
        }

        fn create_window(&self, settings: WindowSettings) -> Window {
            Window::new(TestWindow { settings })
        }
    }

    struct TestWindow {
        settings: WindowSettings,
    }

    impl WindowTrait for TestWindow {
        fn title(&self) -> Result<String, WindowError> {
            Ok(self.settings.title.to_owned())
        }

        fn size(&self) -> Result<(u32, u32), WindowError> {
            Ok(self.settings.size)
        }

        fn close(&self) -> Result<(), WindowError> {
            Err(WindowError::OperationNotSupported)
        }

        fn is_open(&self) -> bool {
            true
        }

        fn redraw(&self) {}
    }

    impl rwh_06::HasWindowHandle for TestWindow {
        fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
            no_handle_panic()
        }
    }

    impl rwh_06::HasDisplayHandle for TestWindow {
        fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
            no_handle_panic()
        }
    }

    #[cfg(feature = "rwh_05")]
    unsafe impl rwh_05::HasRawWindowHandle for TestWindow {
        fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
            no_handle_panic()
        }
    }

    #[cfg(feature = "rwh_05")]
    unsafe impl rwh_05::HasRawDisplayHandle for TestWindow {
        fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
            no_handle_panic()
        }
    }

    fn no_handle_panic() -> ! {
        panic!("TestWindow does not have a Window/Display handle.");
    }

    #[test]
    pub fn should_pump_backend_events_when_event_queue_is_cleared() {
        let ((mut event_queue, _context), test_backend) = TestWindowBackend::init();

        test_backend.buffer_event(WindowEvent::CloseRequested { id: Uuid::new_v4() });

        assert!(
            event_queue.next_event().is_none(),
            "Expected `None`.  Events should be pumped only after a `None` is returned."
        );
        assert!(
            event_queue.next_event().is_some(),
            "Expected `Some`. Events should have been pumped by the previous `next_event()` call."
        );
    }

    #[test]
    pub fn should_create_window_with_backend() {
        let ((mut _event_queue, context), _test_backend) = TestWindowBackend::init();
        let window = context.create_window(
            WindowSettings::default()
                .with_title("Test Window")
                .with_size((1280, 720)),
        );

        assert_eq!(window.title().unwrap(), "Test Window");
        assert_eq!(window.size().unwrap(), (1280, 720));
    }

    #[test]
    pub fn should_impl_send_sync_for_window() {
        let ((mut _event_queue, context), _test_backend) = TestWindowBackend::init();
        let window = context.create_window(WindowSettings::default().with_title("Test Window"));

        thread::scope(|scope| {
            scope
                .spawn(|| {
                    let win = &window;
                    assert_eq!(win.title().unwrap(), "Test Window");
                })
                .join()
                .unwrap();
        });
    }
}
