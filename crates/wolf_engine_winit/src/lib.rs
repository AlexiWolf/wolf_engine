use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};
use wolf_engine_events::{
    dynamic::AnyEvent,
    mpsc::{self, MpscEventReceiver, MpscEventSender},
};
use wolf_engine_window::{backend::WindowSystem, error::WindowError, WindowContext};

pub fn init() -> Result<WinitBackend, WindowError> {
    let (event_sender, event_receiver) = mpsc::event_queue();
    let (window_context, window_event_sender) = WindowContext::new(event_sender.clone());
    match EventLoop::new() {
        Ok(event_loop) => Ok(WinitBackend {
            event_sender,
            event_receiver,
            window_context,
            event_loop,
        }),
        Err(error) => Err(WindowError::InitError(error.to_string())),
    }
}

pub struct WinitBackend {
    event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    event_loop: EventLoop<()>,
}

impl WindowSystem for WinitBackend {
    fn context(&self) -> wolf_engine_window::WindowContext {
        self.window_context.clone()
    }
}

impl wolf_engine_events::event_loop::EventLoop<AnyEvent> for WinitBackend {
    fn event_sender(&self) -> wolf_engine_events::mpsc::MpscEventSender<AnyEvent> {
        todo!()
    }

    fn run<F: FnMut(AnyEvent)>(self, event_handler: F) {}
}

struct WinitApp {}

impl WinitApp {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit()
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}
