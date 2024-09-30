use std::{collections::HashMap, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};
use wolf_engine_events::{
    dynamic::{AnyEvent, AnyEventSender},
    mpsc::{event_queue, MpscEventReceiver, MpscEventSender},
    EventReceiver, EventSender,
};
use wolf_engine_window::{
    error::WindowError,
    event::{BackendEvent, Event},
    raw_window_handle::WindowHandle,
    Uuid, WindowBackend, WindowContext, WindowSettings,
};

pub struct WindowSystem {
    event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    event_loop: EventLoop<()>,
}

impl WindowBackend for WindowSystem {
    fn context(&self) -> WindowContext {
        self.window_context.clone()
    }
}

impl wolf_engine_events::EventLoop<AnyEvent> for WindowSystem {
    fn event_sender(&self) -> MpscEventSender<AnyEvent> {
        self.event_sender.clone()
    }

    fn run<F: FnMut(AnyEvent)>(self, event_handler: F) {
        let mut application = Application::new(
            self.event_sender,
            self.event_receiver,
            self.window_context,
            event_handler,
        );
        let event_loop = self.event_loop;
        let _ = event_loop.run_app(&mut application);
    }
}

struct Application<H: FnMut(AnyEvent)> {
    event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    event_handler: H,

    is_suspended: bool,

    windows: HashMap<Uuid, Arc<Window>>,
    id_map: HashMap<WindowId, Uuid>,
    pending_windows: Vec<(Uuid, WindowSettings)>,
}

impl<H: FnMut(AnyEvent)> Application<H> {
    pub fn new(
        event_sender: MpscEventSender<AnyEvent>,
        event_receiver: MpscEventReceiver<AnyEvent>,
        window_context: WindowContext,
        event_handler: H,
    ) -> Self {
        Self {
            event_sender,
            event_receiver,
            window_context,
            event_handler,

            is_suspended: true,

            windows: HashMap::new(),
            id_map: HashMap::new(),
            pending_windows: Vec::new(),
        }
    }

    fn process_events(&mut self, event_loop: &ActiveEventLoop) {
        while let Some(event) = self.event_receiver.next_event() {
            self.window_context.handle_event(&event);
            if let Some(event) = event.downcast_ref::<BackendEvent>() {
                self.process_backend_events(event, event_loop);
            }

            (self.event_handler)(event);
        }
        (self.event_handler)(Box::new(Event::EventsCleared));
    }

    fn process_backend_events(&mut self, event: &BackendEvent, _event_loop: &ActiveEventLoop) {
        match event {
            BackendEvent::WindowDropped(uuid) => {
                self.windows.remove(uuid);
            }
            BackendEvent::CreateWindow(uuid, settings) => self
                .pending_windows
                .push((uuid.to_owned(), settings.to_owned())),
            _ => (),
        }
    }

    fn create_windows(&mut self, event_loop: &ActiveEventLoop) {
        if self.is_suspended {
            return;
        }

        while let Some((uuid, settings)) = self.pending_windows.pop() {
            let window_attributes = WindowAttributes::default()
                .with_title(settings.title)
                .with_inner_size(PhysicalSize::new(settings.size.0, settings.size.1))
                .with_visible(settings.is_visible)
                .with_resizable(settings.is_resizable);
            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Window creation should have succeeded"),
            );
            let window_handle = WindowHandle::new(window.clone());

            self.id_map.insert(window.id(), uuid);
            self.windows.insert(uuid, window);
            self.window_context
                .window(uuid)
                .expect("window should have been created by the window context")
                .set_handle(window_handle);

            (self.event_handler)(Box::new(Event::WindowEvent(
                uuid,
                wolf_engine_window::event::WindowEvent::Ready(Ok(())),
            )))
        }
    }
}

impl<H: FnMut(AnyEvent)> ApplicationHandler for Application<H> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.is_suspended = false;
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.is_suspended = true;
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        event_loop.set_control_flow(ControlFlow::Poll);
        match cause {
            StartCause::Init => (self.event_handler)(Box::new(Event::Started)),
            StartCause::Poll => {
                self.process_events(event_loop);
                self.create_windows(event_loop);
            }
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        let uuid = self.id_map.get(&window_id).unwrap();
        match event {
            winit::event::WindowEvent::CloseRequested => self
                .event_sender
                .send_any_event(BackendEvent::WindowDropped(uuid.to_owned()))
                .unwrap(),
            _ => (),
        }
    }
}

pub fn init() -> Result<WindowSystem, WindowError> {
    let (event_sender, event_receiver) = event_queue();
    let window_context = wolf_engine_window::init(event_sender.clone());
    let winit_event_loop = EventLoop::new().unwrap();
    Ok(WindowSystem {
        window_context,
        event_sender,
        event_receiver,
        event_loop: winit_event_loop,
    })
}
