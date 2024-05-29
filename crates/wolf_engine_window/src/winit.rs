use wolf_engine_events::mpsc::MpscEventSender;

use crate::*;

#[derive(Copy, Clone)]
pub struct WinitBackend;

impl WindowBackend for WinitBackend {
    fn init(self, event_sender: MpscEventSender<WindowEvent>) -> Box<dyn WindowBackendAdapter> {
        todo!()
    }
}
