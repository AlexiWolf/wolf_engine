use winit::{
    application::ApplicationHandler,
    event::StartCause,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};
use wolf_engine_events::{dynamic::AnyEvent, mpsc::MpscEventReceiver};
use wolf_engine_window::{error::WindowError, event::Event};

pub fn run<E: FnMut(Event)>(
    event_receiver: MpscEventReceiver<AnyEvent>,
    event_handler: E,
) -> Result<(), WindowError> {
    let event_loop = EventLoop::new().unwrap();
    let mut winit_adapter = WinitAdapter::new();
    let _ = event_loop.run_app(&mut winit_adapter);
    Ok(())
}

struct WinitAdapter {}

impl WinitAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApplicationHandler for WinitAdapter {
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
        runner: Box::new(|_| {}),
    };
    Ok(WindowSystem {
        application,
        event_loop: winit_event_loop,
    })
}
