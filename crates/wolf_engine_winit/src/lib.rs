use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};
use wolf_engine_events::{
    dynamic::AnyEvent,
    mpsc::{self, MpscEventReceiver, MpscEventSender},
    EventReceiver,
};
use wolf_engine_window::{
    backend::WindowSystem,
    error::WindowError,
    event::{WindowContextEvent, WindowEvent},
    WindowContext, WindowContextEventSender,
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
        let mut winit_app = WinitApp::new(
            event_handler,
            self.event_sender,
            self.event_receiver,
            self.window_context,
            self.window_context_event_sender,
        );
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
    is_suspended: bool,
}

impl<H: FnMut(AnyEvent)> WinitApp<H> {
    pub(crate) fn new(
        event_handler: H,
        event_sender: MpscEventSender<AnyEvent>,
        event_receiver: MpscEventReceiver<AnyEvent>,
        window_context: WindowContext,
        window_context_event_sender: WindowContextEventSender,
    ) -> Self {
        Self {
            event_handler,
            event_sender,
            event_receiver,
            window_context,
            window_context_event_sender,
            is_suspended: true,
        }
    }

    fn process_events(&mut self, event_loop: &ActiveEventLoop) {
        while let Some(event) = self.event_receiver.next_event() {
            self.handle_event(event_loop, event);
        }
    }

    fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: AnyEvent) {
        if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
            match context_event {
                WindowContextEvent::WindowCreated(_, _) => (),
                WindowContextEvent::WindowClosed(_) => (),
                _ => (),
            }
        }
    }
}

impl<H: FnMut(AnyEvent)> ApplicationHandler for WinitApp<H> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.is_suspended = false;
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.is_suspended = true;
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        event_loop.set_control_flow(ControlFlow::Poll);
        match cause {
            StartCause::Init => (self.event_handler)(Box::new(WindowEvent::Started)),
            StartCause::Poll => {
                self.process_events(event_loop);
            }
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}
