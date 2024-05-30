use std::{collections::HashMap, sync::RwLock, time::Duration};

use ::winit::dpi::PhysicalSize;
use ::winit::platform::pump_events::EventLoopExtPumpEvents;
use ::winit::window::WindowAttributes;
use ::winit::{event_loop::EventLoop, window::WindowId};

use wolf_engine_events::mpsc::MpscEventSender;

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
}

impl WinitAdapter {
    fn new(event_sender: MpscEventSender<WindowEvent>, event_loop: EventLoop<()>) -> Self {
        Self {
            event_sender,
            event_loop: RwLock::new(event_loop),
            window_uuids: RwLock::new(HashMap::new()),
        }
    }
}

impl WindowBackendAdapter for WinitAdapter {
    fn pump_events(&self) {
        let timeout = Duration::ZERO;
        #[allow(deprecated)]
        self.event_loop.write().unwrap().pump_events(
            Some(timeout),
            |event, event_loop| match event {
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
        let window = Window::new(winit_window);
        window
    }
}
