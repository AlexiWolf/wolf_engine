use std::{collections::HashMap, sync::RwLock, time::Duration};

use ::winit::dpi::PhysicalSize;
use ::winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use ::winit::platform::pump_events::EventLoopExtPumpEvents;
use ::winit::window::{Window as WinitWindow, WindowAttributes};
use ::winit::{event_loop::EventLoop, window::WindowId};
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;

use wolf_engine_events::mpsc::{self, MpscEventSender};
use wolf_engine_events::EventSender;

use crate::backend::{WindowBackend, WindowTrait};
use crate::event::WindowEvent;
use crate::{Window, WindowContext, WindowEventQueue, WindowSettings, WindowSystem};

use super::WindowResult;

enum BackendEvent {
    WindowDropped { id: WindowId },
}

pub struct WinitBackend {
    event_sender: MpscEventSender<WindowEvent>,
    event_loop: RwLock<EventLoop<BackendEvent>>,
    window_uuids: RwLock<HashMap<WindowId, Uuid>>,
}

impl WinitBackend {
    pub fn init() -> Result<WindowSystem, &'static str> {
        let event_loop = match EventLoop::with_user_event().build() {
            Ok(event_loop) => event_loop,
            Err(_) => return Err("Failed to initialize the window system"),
        };
        let (event_sender, event_receiver) = mpsc::event_queue();
        let winit_adapter = WinitBackend::new(event_sender, event_loop);
        let context = WindowContext::new(Box::new(winit_adapter));
        let event_queue = WindowEventQueue::new(&context, event_receiver);
        Ok((event_queue, context))
    }

    fn new(
        event_sender: MpscEventSender<WindowEvent>,
        event_loop: EventLoop<BackendEvent>,
    ) -> Self {
        Self {
            event_sender,
            event_loop: RwLock::new(event_loop),
            window_uuids: RwLock::new(HashMap::new()),
        }
    }

    fn insert_id(&self, winit_id: WindowId, uuid: Uuid) {
        self.window_uuids.write().unwrap().insert(winit_id, uuid);
    }

    fn get_uuid(&self, winit_id: WindowId) -> Option<Uuid> {
        self.window_uuids.read().unwrap().get(&winit_id).copied()
    }

    fn remove_id(&self, winit_id: WindowId) {
        self.window_uuids.write().unwrap().remove(&winit_id);
    }
}

impl WindowBackend for WinitBackend {
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
                WinitEvent::WindowEvent {
                    window_id,
                    event: WinitWindowEvent::RedrawRequested,
                } => {
                    if let Some(uuid) = self.get_uuid(window_id) {
                        self.event_sender
                            .send_event(WindowEvent::RedrawRequested { id: uuid })
                            .unwrap();
                    }
                }
                WinitEvent::UserEvent(BackendEvent::WindowDropped { id }) => self.remove_id(id),
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
        let event_loop_proxy = self.event_loop.read().unwrap().create_proxy();
        let window_handle = WinitWindowHandle::new(winit_window, event_loop_proxy);
        let window = Window::new(window_handle);
        let window_uuid = window.id();
        self.insert_id(winit_id, window_uuid);
        window
    }
}

struct WinitWindowHandle {
    inner: WinitWindow,
    event_loop: EventLoopProxy<BackendEvent>,
}

impl WinitWindowHandle {
    pub fn new(window: WinitWindow, event_loop: EventLoopProxy<BackendEvent>) -> Self {
        Self {
            inner: window,
            event_loop,
        }
    }
}

impl Drop for WinitWindowHandle {
    fn drop(&mut self) {
        let id = self.inner.id();
        let _ = self
            .event_loop
            .send_event(BackendEvent::WindowDropped { id });
    }
}

impl WindowTrait for WinitWindowHandle {
    fn title(&self) -> WindowResult<String> {
        Ok(self.inner.title())
    }

    fn size(&self) -> WindowResult<(u32, u32)> {
        let size = self.inner.inner_size();
        Ok((size.width, size.height))
    }

    fn redraw(&self) -> WindowResult<()> {
        self.inner.request_redraw();
        Ok(())
    }
}

impl rwh_06::HasWindowHandle for WinitWindowHandle {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasWindowHandle::window_handle(&self.inner)
    }
}

impl rwh_06::HasDisplayHandle for WinitWindowHandle {
    fn display_handle(&self) -> Result<rwh_06::DisplayHandle<'_>, rwh_06::HandleError> {
        rwh_06::HasDisplayHandle::display_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for WinitWindowHandle {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        rwh_05::HasRawWindowHandle::raw_window_handle(&self.inner)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawDisplayHandle for WinitWindowHandle {
    fn raw_display_handle(&self) -> rwh_05::RawDisplayHandle {
        rwh_05::HasRawDisplayHandle::raw_display_handle(&self.inner)
    }
}

#[cfg(test)]
#[cfg(any(target_os = "windows", target_os = "linux"))]
mod winit_backend_tests {
    use super::*;

    #[cfg(target_os = "windows")]
    use winit::platform::windows::EventLoopBuilderExtWindows;

    #[cfg(target_os = "linux")]
    use winit::platform::x11::EventLoopBuilderExtX11;

    #[test]
    fn should_clean_up_window_data_after_window_drop() {
        let event_loop = EventLoop::with_user_event()
            .with_any_thread(true)
            .build()
            .expect("Failed to create Winit's EventLoop");
        let (event_sender, _event_receiver) = mpsc::event_queue();
        let winit_backend = WinitBackend::new(event_sender, event_loop);

        assert_eq!(
            known_window_count(&winit_backend),
            0,
            "Expected 0 known window IDs, as no windows have been created yet"
        );
        winit_backend.pump_events();

        let window = winit_backend.create_window(WindowSettings::default().is_hidden());
        winit_backend.pump_events();
        assert_eq!(
            known_window_count(&winit_backend),
            1,
            "Expected 1 known window ID, as a window has been created"
        );

        drop(window);
        winit_backend.pump_events();
        assert_eq!(
            known_window_count(&winit_backend),
            0,
            "Expected 0 known window IDs, as the ID should have been removed when the window was dropped"
        );
    }

    fn known_window_count(backend: &WinitBackend) -> usize {
        backend.window_uuids.read().unwrap().len()
    }
}
