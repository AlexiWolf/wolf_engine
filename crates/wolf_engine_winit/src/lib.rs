use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};
use wolf_engine_events::{
    dynamic::AnyEvent,
    mpsc::{event_queue, MpscEventReceiver, MpscEventSender},
};
use wolf_engine_window::{error::WindowError, event::Event, WindowBackend, WindowContext};

pub struct WindowSystem {
    application: Application,
    event_loop: EventLoop<()>,
}

impl WindowBackend for WindowSystem {
    fn context(&self) -> WindowContext {
        self.application.window_context.clone()
    }
}

impl wolf_engine_events::EventLoop<AnyEvent> for WindowSystem {
    fn event_sender(&self) -> MpscEventSender<AnyEvent> {
        self.application.event_sender.clone()
    }

    fn run<F: FnMut(AnyEvent)>(self, _event_handler: F) {
        let mut application = self.application;
        let event_loop = self.event_loop;
        let _ = event_loop.run_app(&mut application);
    }
}

struct Application {
    event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    event_handler: Box<dyn FnMut(Event)>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

pub fn init() -> Result<WindowSystem, WindowError> {
    let (event_sender, event_receiver) = event_queue();
    let window_context = wolf_engine_window::init(event_sender.clone());
    let winit_event_loop = EventLoop::new().unwrap();
    let application = Application {
        event_sender,
        event_receiver,
        window_context,
        event_handler: Box::new(|_| {}),
    };
    Ok(WindowSystem {
        application,
        event_loop: winit_event_loop,
    })
}
