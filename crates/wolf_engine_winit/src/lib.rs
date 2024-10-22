use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};
use wolf_engine_events::{
    dynamic::AnyEvent,
    mpsc::{self, MpscEventReceiver, MpscEventSender},
};
use wolf_engine_window::{
    backend::WindowSystem, error::WindowError, WindowContext, WindowContextEventSender,
};

pub fn init() -> Result<WinitBackend, WindowError> {
    let (event_sender, event_receiver) = mpsc::event_queue();
    let (window_context, window_context_event_sender) = WindowContext::new(event_sender.clone());
    match EventLoop::new() {
        Ok(event_loop) => Ok(WinitBackend {
            event_sender,
            event_receiver,
            window_context,
            window_context_event_sender,
            event_loop,
        }),
        Err(error) => Err(WindowError::InitError(error.to_string())),
    }
}

pub struct WinitBackend {
    event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    window_context_event_sender: WindowContextEventSender,
    event_loop: EventLoop<()>,
}

impl WindowSystem for WinitBackend {
    fn context(&self) -> WindowContext {
        self.window_context.clone()
    }
}

impl wolf_engine_events::event_loop::EventLoop<AnyEvent> for WinitBackend {
    fn event_sender(&self) -> MpscEventSender<AnyEvent> {
        self.event_sender.clone()
    }

    fn run<F: FnMut(AnyEvent)>(self, event_handler: F) {
        let mut winit_app = WinitApp {
            event_handler,
            event_sender: self.event_sender,
            event_receiver: self.event_receiver,
            window_context: self.window_context,
            window_context_event_sender: self.window_context_event_sender,
        };
        let event_loop = self.event_loop;

        let _ = event_loop.run_app(&mut winit_app);
    }
}

struct WinitApp<H: FnMut(AnyEvent)> {
    event_handler: H,
    event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    window_context_event_sender: WindowContextEventSender,
}

impl<H: FnMut(AnyEvent)> ApplicationHandler for WinitApp<H> {
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
