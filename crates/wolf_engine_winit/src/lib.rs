use std::{collections::HashMap, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{StartCause, WindowEvent as WinitWindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};
use wolf_engine_events::{
    dynamic::{AnyEvent, AnyEventSender, Event},
    mpsc::{event_queue, MpscEventReceiver, MpscEventSender},
    EventReceiver,
};
use wolf_engine_window::{
    error::WindowError,
    event::{BackendEvent, WindowEvent, WindowEvent as WeWindowEvent},
    raw_window_handle::WindowHandle,
    Uuid, WindowBackend, WindowContext, WindowSettings,
};

pub fn init() -> Result<WindowSystem, WindowError> {
    let (event_sender, event_receiver) = event_queue();
    let window_context = WindowContext::new(event_sender.clone());
    match EventLoop::new() {
        Ok(event_loop) => Ok(WindowSystem {
            window_context,
            event_sender,
            event_receiver,
            event_loop,
        }),
        Err(error) => Err(WindowError::InitError(error.to_string())),
    }
}

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

            if let Some(window_event) = event.downcast_ref::<WindowEvent>() {
                match window_event {
                    WindowEvent::Exited => event_loop.exit(),
                    _ => (),
                }
            }
            (self.event_handler)(event);
        }
        (self.event_handler)(Box::new(WindowEvent::EventsCleared));
    }

    fn process_backend_events(&mut self, event: &BackendEvent, _event_loop: &ActiveEventLoop) {
        match event {
            BackendEvent::WindowDropped(uuid) => match self.windows.remove(uuid) {
                Some(window) => {
                    let _ = self.id_map.remove(&window.id());
                }
                None => (),
            },
            BackendEvent::CreateWindow(uuid, settings) => self
                .pending_windows
                .push((uuid.to_owned(), settings.to_owned())),
            BackendEvent::RedrawRequested(uuid) => {
                if let Some(window) = self.windows.get(uuid) {
                    window.request_redraw();
                }
            }
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

            (self.event_handler)(Box::new(WeWindowEvent::WindowReady(uuid, Ok(()))))
        }
    }

    fn send_event_buffered<E: Event>(&self, event: E) {
        self.event_sender.send_any_event(event).unwrap();
    }

    fn send_event_immediately<E: Event>(&mut self, event: E) {
        let event = Box::new(event) as AnyEvent;
        self.window_context.handle_event(&event);
        (self.event_handler)(event);
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
            StartCause::Init => (self.event_handler)(Box::new(WindowEvent::Started)),
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
        event: WinitWindowEvent,
    ) {
        let uuid = match self.id_map.get(&window_id) {
            Some(uuid) => uuid,
            None => return,
        }
        .to_owned();
        match event {
            WinitWindowEvent::CloseRequested => self
                .event_sender
                .send_any_event(WeWindowEvent::WindowCloseRequested(uuid))
                .unwrap(),
            WinitWindowEvent::RedrawRequested => {
                (self.event_handler)(Box::new(WeWindowEvent::WindowRedrawRequested(uuid)))
            }
            WinitWindowEvent::Resized(new_size) => self.send_event_immediately(
                WeWindowEvent::WindowResized(uuid, (new_size.width, new_size.height)),
            ),
            _ => (),
        }
    }
}
