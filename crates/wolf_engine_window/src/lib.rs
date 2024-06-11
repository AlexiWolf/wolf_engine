use std::sync::Arc;

use uuid::Uuid;
use winit::WinitBackend;
use wolf_engine_events::{
    mpsc::{self, MpscEventReceiver, MpscEventSender},
    EventReceiver,
};

mod winit;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    CloseRequested { id: Uuid },
    Resized { id: Uuid, width: u32, height: u32 },
}

pub type WindowSystem = (EventQueue, Context);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WindowSettings {
    pub title: &'static str,
    pub size: (u32, u32),
}

impl WindowSettings {
    pub fn with_title<T: Into<&'static str>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size<T: Into<(u32, u32)>>(mut self, size: T) -> Self {
        self.size = size.into();
        self
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "untitled",
            size: (800, 600),
        }
    }
}

pub struct Window {
    id: Uuid,
    inner: Box<dyn WindowTrait>,
}

impl Window {
    pub fn new<T: WindowTrait + 'static>(inner: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: Box::new(inner),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> Result<String, WindowError> {
        self.inner.title()
    }

    pub fn size(&self) -> Result<(u32, u32), WindowError> {
        self.inner.size()
    }

    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    pub fn close(&self) {
        self.inner.close()
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasWindowHandle::window_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(&*self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(&*self.inner)
    }
}

#[cfg(feature = "rwh_06")]
pub trait HasRwh6Handles: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}
#[cfg(feature = "rwh_06")]
impl<T> HasRwh6Handles for T where T: rwh_06::HasWindowHandle + rwh_06::HasDisplayHandle {}

#[cfg(not(feature = "rwh_06"))]
pub trait HasRwh6Handles {}
#[cfg(not(feature = "rwh_06"))]
impl<T> HasRwh6Handles for T {}

#[cfg(feature = "rwh_05")]
pub trait HasRwh5Handles: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}
#[cfg(feature = "rwh_05")]
impl<T> HasRwh5Handles for T where T: rwh_05::HasRawWindowHandle + rwh_05::HasRawDisplayHandle {}

#[cfg(not(feature = "rwh_05"))]
pub trait HasRwh5Handles {}
#[cfg(not(feature = "rwh_05"))]
impl<T> HasRwh5Handles for T {}

pub trait HasRawWindowHandles: HasRwh6Handles + HasRwh5Handles {}
impl<T> HasRawWindowHandles for T where T: HasRwh6Handles + HasRwh5Handles {}

pub trait WindowTrait: HasRawWindowHandles + Send + Sync {
    fn title(&self) -> Result<String, WindowError>;
    fn size(&self) -> Result<(u32, u32), WindowError>;
    fn is_open(&self) -> bool;
    fn close(&self);
}

pub struct EventQueue {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
    event_receiver: MpscEventReceiver<WindowEvent>,
}

impl EventQueue {
    fn new(context: &Context, event_receiver: MpscEventReceiver<WindowEvent>) -> Self {
        let backend_adapter = context.backend_adapter();
        Self {
            backend_adapter,
            event_receiver,
        }
    }
}

impl EventReceiver<WindowEvent> for EventQueue {
    fn next_event(&mut self) -> Option<WindowEvent> {
        let event = self.event_receiver.next_event();
        if event.is_none() {
            self.backend_adapter.pump_events();
        }
        event
    }
}

pub struct Context {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
}

impl Context {
    fn new(backend_adapter: Box<dyn WindowBackendAdapter>) -> Self {
        Self {
            backend_adapter: Arc::new(backend_adapter),
        }
    }

    pub fn create_window(&self, settings: WindowSettings) -> Window {
        self.backend_adapter.create_window(settings)
    }

    fn backend_adapter(&self) -> Arc<Box<dyn WindowBackendAdapter>> {
        self.backend_adapter.clone()
    }
}

pub fn init() -> Result<WindowSystem, &'static str> {
    init_with_backend(WinitBackend)
}

pub fn init_with_backend<T: WindowBackend + 'static>(
    backend: T,
) -> Result<WindowSystem, &'static str> {
    let (event_sender, event_receiver) = mpsc::event_queue();
    let backend_adapter = backend.init(event_sender);
    let context = Context::new(backend_adapter);
    let event_queue = EventQueue::new(&context, event_receiver);
    Ok((event_queue, context))
}

pub trait WindowBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter>;
}

pub trait WindowBackendAdapter {
    fn pump_events(&self);
    fn create_window(&self, settings: WindowSettings) -> Window;
}

#[cfg(test)]
mod window_system_tests {
    use std::{collections::VecDeque, sync::RwLock, thread};

    use wolf_engine_events::EventSender;

    use super::*;

    struct TestWindowBackend {
        adapter: TestWindowBackendAdapter,
    }

    impl TestWindowBackend {
        pub fn new(adapter: TestWindowBackendAdapter) -> Self {
            Self { adapter }
        }
    }

    impl WindowBackend for TestWindowBackend {
        fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter> {
            *self.adapter.event_sender.write().unwrap() = Some(event_sender);
            Box::new(self.adapter)
        }
    }

    #[derive(Clone)]
    pub struct TestWindowBackendAdapter {
        event_sender: Arc<RwLock<Option<MpscEventSender<WindowEvent>>>>,
        pub buffered_events: Arc<RwLock<VecDeque<WindowEvent>>>,
    }

    impl TestWindowBackendAdapter {
        fn new() -> Self {
            Self {
                event_sender: Arc::new(RwLock::new(None)),
                buffered_events: Arc::new(RwLock::new(VecDeque::new())),
            }
        }

        pub fn buffer_event(&self, event: WindowEvent) {
            self.buffered_events.write().unwrap().push_back(event);
        }
    }

    impl WindowBackendAdapter for TestWindowBackendAdapter {
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

        fn is_open(&self) -> bool {
            todo!()
        }

        fn close(&self) {
            todo!()
        }
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
        let test_adapter = TestWindowBackendAdapter::new();
        let (mut event_queue, _context) =
            crate::init_with_backend(TestWindowBackend::new(test_adapter.clone())).unwrap();

        test_adapter.buffer_event(WindowEvent::CloseRequested { id: Uuid::new_v4() });

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
        let test_adapter = TestWindowBackendAdapter::new();
        let (mut _event_queue, context) =
            crate::init_with_backend(TestWindowBackend::new(test_adapter.clone())).unwrap();
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
        let test_adapter = TestWindowBackendAdapter::new();
        let (mut _event_queue, context) =
            crate::init_with_backend(TestWindowBackend::new(test_adapter)).unwrap();
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
