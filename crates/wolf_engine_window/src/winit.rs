use std::sync::Weak;
use std::{collections::HashMap, sync::RwLock, time::Duration};

use ::winit::dpi::PhysicalSize;
use ::winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use ::winit::platform::pump_events::EventLoopExtPumpEvents;
use ::winit::window::{Window as WinitWindow, WindowAttributes};
use ::winit::{event_loop::EventLoop, window::WindowId};

use wolf_engine_events::mpsc::MpscEventSender;
use wolf_engine_events::EventSender;

use crate::*;

#[derive(Copy, Clone)]
pub struct WinitBackend;

impl WindowBackend for WinitBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter> {
        let event_loop = EventLoop::new().unwrap();
        let winit_adapter = WinitAdapter::new(event_sender, event_loop);
        Box::from(winit_adapter)
    }
}

pub struct WinitAdapter {
    event_sender: MpscEventSender<WindowEvent>,
    event_loop: RwLock<EventLoop<()>>,
    window_uuids: RwLock<HashMap<WindowId, Uuid>>,
    windows: RwLock<HashMap<Uuid, Arc<WinitWindow>>>,
}

impl WinitAdapter {
    fn new(event_sender: MpscEventSender<WindowEvent>, event_loop: EventLoop<()>) -> Self {
        Self {
            event_sender,
            event_loop: RwLock::new(event_loop),
            window_uuids: RwLock::new(HashMap::new()),
            windows: RwLock::new(HashMap::new()),
        }
    }

    fn insert_id(&self, winit_id: WindowId, uuid: Uuid) {
        self.window_uuids.write().unwrap().insert(winit_id, uuid);
    }

    fn get_uuid(&self, winit_id: WindowId) -> Option<Uuid> {
        self.window_uuids.read().unwrap().get(&winit_id).copied()
    }
}

impl WindowBackendAdapter for WinitAdapter {
    fn pump_events(&self) {
        let timeout = Duration::ZERO;
        #[allow(deprecated)]
        self.event_loop.write().unwrap().pump_events(
            Some(timeout),
            |event, _event_loop| match event {
                WinitEvent::WindowEvent {
                    window_id,
                    event: WinitWindowEvent::CloseRequested,
                } => {
                    if let Some(uuid) = self.get_uuid(window_id) {
                        self.event_sender
                            .send_event(WindowEvent::CloseRequested { id: uuid })
                            .unwrap();
                    }
                }
                WinitEvent::WindowEvent {
                    window_id,
                    event: WinitWindowEvent::Resized(size),
                } => {
                    if let Some(uuid) = self.get_uuid(window_id) {
                        self.event_sender
                            .send_event(WindowEvent::Resized {
                                id: uuid,
                                width: size.width,
                                height: size.height,
                            })
                            .unwrap();
                    }
                }
                _ => (),
            },
        );
    }

    fn create_window(&self, settings: WindowSettings) -> Window {
        #[allow(deprecated)]
        let winit_window = self
            .event_loop
            .read()
            .unwrap()
            .create_window(
                WindowAttributes::default()
                    .with_title(settings.title)
                    .with_inner_size(PhysicalSize::new(settings.size.0, settings.size.1)),
            )
            .unwrap();
        let winit_id = winit_window.id();
        let window = Window::new(winit_window);
        let window_uuid = window.id();
        self.insert_id(winit_id, window_uuid);
        window
    }
}

struct WinitWindowHandle {
    inner: Weak<::winit::window::Window>,
}

impl WindowTrait for WinitWindowHandle {
    fn title(&self) -> String {
        todo!()
    }

    fn size(&self) -> (u32, u32) {
        todo!()
    }

    fn is_open(&self) -> bool {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl rwh_06::HasWindowHandle for WinitWindowHandle {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        todo!()
    }
}

impl rwh_06::HasDisplayHandle for WinitWindowHandle {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        todo!()
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for WinitWindowHandle {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        todo!()
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for WinitWindowHandle {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        todo!()
    }
}

impl WindowTrait for ::winit::window::Window {
    fn title(&self) -> String {
        ::winit::window::Window::title(&self)
    }

    fn size(&self) -> (u32, u32) {
        let size = self.inner_size();
        (size.height, size.width)
    }

    fn is_open(&self) -> bool {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}
