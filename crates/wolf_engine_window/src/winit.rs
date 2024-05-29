use std::{collections::HashMap, sync::RwLock};

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
        todo!()
    }
}
