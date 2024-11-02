use std::{collections::HashMap, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{StartCause, WindowEvent as WinitEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};
use wolf_engine_events::{
    dynamic::AnyEvent,
    mpsc::{self, MpscEventReceiver, MpscEventSender},
    EventReceiver, EventSender,
};
use wolf_engine_window::{
    backend::{
        event::{WindowContextEvent, WindowContextEventSender},
        WindowSystem,
    },
    raw_window_handle::WindowHandle,
    Uuid, WindowContext, WindowError, WindowEvent, WindowSettings,
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
    _event_sender: MpscEventSender<AnyEvent>,
    event_receiver: MpscEventReceiver<AnyEvent>,
    window_context: WindowContext,
    window_context_event_sender: WindowContextEventSender,
    is_suspended: bool,

    pending_windows: Vec<(Uuid, WindowSettings)>,
    id_map: HashMap<WindowId, Uuid>,
    windows: HashMap<Uuid, Arc<Window>>,
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
            _event_sender: event_sender,
            event_receiver,
            window_context,
            window_context_event_sender,
            is_suspended: true,

            pending_windows: Vec::new(),
            id_map: HashMap::new(),
            windows: HashMap::new(),
        }
    }

    fn process_events(&mut self, event_loop: &ActiveEventLoop) {
        while let Some(event) = self.event_receiver.next_event() {
            self.handle_event(event_loop, event);
        }
        (self.event_handler)(Box::new(WindowEvent::EventsCleared));
    }

    fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: AnyEvent) {
        if let Some(context_event) = event.downcast_ref::<WindowContextEvent>() {
            match context_event {
                WindowContextEvent::Exited => {
                    event_loop.exit();
                    (self.event_handler)(Box::new(WindowEvent::Exited))
                }
                WindowContextEvent::WindowCreated(uuid, settings) => self
                    .pending_windows
                    .push((uuid.to_owned(), settings.to_owned())),

                WindowContextEvent::WindowRedrawRequested(uuid) => {
                    if let Some(window) = self.windows.get(uuid) {
                        window.request_redraw();
                    }
                }
                WindowContextEvent::WindowClosed(uuid) => {
                    if let Some(window) = self.windows.remove(uuid) {
                        let _ = self.id_map.remove(&window.id());
                    }
                }
                _ => (),
            }
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
                .insert_window_handle(uuid, window_handle);
            (self.event_handler)(Box::new(WindowEvent::WindowReady(uuid, Ok(()))));
        }
    }

    fn resize_window(&mut self, uuid: Uuid, width: u32, height: u32) {
        self.window_context_event_sender
            .send_event(WindowContextEvent::WindowResized(uuid, width, height))
            .unwrap();
        (self.event_handler)(Box::new(WindowEvent::WindowResized(uuid, width, height)))
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
        let uuid = match self.id_map.get(&window_id) {
            Some(uuid) => uuid,
            None => return,
        }
        .to_owned();
        match event {
            WinitEvent::CloseRequested => {
                (self.event_handler)(Box::new(WindowEvent::WindowClosed(uuid)))
            }
            WinitEvent::Resized(new_size) => {
                self.resize_window(uuid, new_size.width, new_size.height)
            }
            WinitEvent::RedrawRequested => {
                (self.event_handler)(Box::new(WindowEvent::WindowRedrawRequested(uuid)))
            }
            _ => (),
        }
    }
}
