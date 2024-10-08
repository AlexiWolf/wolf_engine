//! Provides an event system for the window crate.

use anyhow::anyhow;
use uuid::Uuid;
use winit::{
    error::EventLoopError,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Fullscreen, WindowAttributes},
};
use wolf_engine_events::{
    mpsc::{self, MpscEventReceiver},
    EventReceiver,
};
use wolf_engine_input::{Input, ToInput};

use crate::{
    error::{OsError, UnsupportedError, WindowError},
    FullscreenMode, Window, WindowContext, WindowIdMap, WindowSettings,
};

/// An event generated by the window system.
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum Event {
    Started,
    EventsCleared,
    WindowEvent(Uuid, WindowEvent),
    Input(Input),
    Exited,
}

/// An event generated by a specific window.
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum WindowEvent {
    Created(Result<Window, WindowError>),
    RedrawRequested,
    Resized(u32, u32),
    Closed,
    Input(Input),
}

pub(crate) enum BackendEvent {
    CreateWindow(Uuid, WindowSettings),
    WindowDropped(Uuid),
    Exit,
}

pub(crate) type WinitEventLoop = winit::event_loop::EventLoop<()>;

/// Provides a way to configure the window system.
///
/// Create a new builder by calling [`init()`](crate::init()).
pub struct EventLoopBuilder {}

impl EventLoopBuilder {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Initialize the window system.
    pub fn build(self) -> Result<(EventLoop, WindowContext), WindowError> {
        match WinitEventLoop::new() {
            Ok(event_loop) => {
                let (event_sender, event_receiver) = mpsc::event_queue();
                let context = WindowContext::new(event_sender);
                let event_loop = EventLoop::new(event_receiver, event_loop, context.clone());
                Ok((event_loop, context))
            }
            Err(error) => match error {
                EventLoopError::Os(error) => {
                    Err(WindowError::OsError(OsError::from(anyhow!(error))))
                }
                EventLoopError::RecreationAttempt => Err(UnsupportedError::from(anyhow!(
                    "Only 1 EventLoop can exist at a time"
                ))
                .into()),
                error => panic!("Unhandled Error: {error}"),
            },
        }
    }
}

/// The main-loop of the window system.
pub struct EventLoop {
    event_receiver: MpscEventReceiver<BackendEvent>,
    event_loop: Option<WinitEventLoop>,
    context: WindowContext,
    is_first_resume: bool,
    window_ids: WindowIdMap,
}

impl EventLoop {
    fn new(
        event_receiver: MpscEventReceiver<BackendEvent>,
        event_loop: WinitEventLoop,
        context: WindowContext,
    ) -> Self {
        Self {
            event_receiver,
            event_loop: Some(event_loop),
            context,
            is_first_resume: true,
            window_ids: WindowIdMap::new(),
        }
    }
}

impl EventLoop {
    /// Run the main-loop, passing events to the provided callback.
    #[allow(deprecated)]
    pub fn run<F: FnMut(Event)>(mut self, mut event_handler: F) {
        let event_loop = std::mem::take(&mut self.event_loop)
            .expect("event-loop is present, because this method can only be run once");
        let _ = event_loop.run(|event, event_loop| {
            while let Some(event) = self.event_receiver.next_event() {
                match event {
                    BackendEvent::CreateWindow(uuid, window_settings) => {
                        let window_result = self.create_window(uuid, event_loop, window_settings);
                        (event_handler)(Event::WindowEvent(
                            uuid,
                            WindowEvent::Created(window_result),
                        ));
                    }
                    BackendEvent::WindowDropped(uuid) => {
                        self.window_ids.remove(uuid);
                    }
                    BackendEvent::Exit => event_loop.exit(),
                }
            }

            match event {
                WinitEvent::AboutToWait => {
                    (event_handler)(Event::EventsCleared);
                    event_loop.set_control_flow(ControlFlow::Poll);
                }
                WinitEvent::Resumed => {
                    if self.is_first_resume {
                        self.is_first_resume = false;
                        (event_handler)(Event::Started);
                    }
                }
                WinitEvent::LoopExiting => (event_handler)(Event::Exited),
                WinitEvent::DeviceEvent { event, .. } => {
                    if let Some(input) = event.to_input() {
                        (event_handler)(Event::Input(input));
                    }
                }
                WinitEvent::WindowEvent {
                    window_id,
                    event: window_event,
                } => {
                    let uuid = match self.window_ids.uuid_of(window_id) {
                        Some(uuid) => uuid,
                        None => return,
                    };

                    if let Some(input) = window_event.to_input() {
                        (event_handler)(Event::WindowEvent(uuid, WindowEvent::Input(input)));
                    }

                    match window_event {
                        WinitWindowEvent::RedrawRequested => {
                            (event_handler)(Event::WindowEvent(uuid, WindowEvent::RedrawRequested))
                        }
                        WinitWindowEvent::Resized(size) => (event_handler)(Event::WindowEvent(
                            uuid,
                            WindowEvent::Resized(size.width, size.height),
                        )),
                        WinitWindowEvent::CloseRequested => {
                            (event_handler)(Event::WindowEvent(uuid, WindowEvent::Closed))
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });
    }

    fn create_window(
        &mut self,
        uuid: Uuid,
        event_loop: &ActiveEventLoop,
        window_settings: WindowSettings,
    ) -> Result<Window, WindowError> {
        let fullscreen_mode = window_settings.fullscreen_mode.clone();
        let mut window_attributes: WindowAttributes = window_settings.into();
        if let Some(fullscreen_mode) = fullscreen_mode {
            let monitor_handle = event_loop.primary_monitor();
            match fullscreen_mode {
                FullscreenMode::Borderless => {
                    window_attributes = window_attributes
                        .with_fullscreen(Some(Fullscreen::Borderless(monitor_handle)))
                }
            }
        }
        match event_loop.create_window(window_attributes) {
            Ok(winit_window) => {
                let window = Window::new(uuid, self.context.event_sender(), winit_window);
                self.window_ids.insert(&window);
                Ok(window)
            }
            Err(error) => Err(WindowError::OsError(OsError::from(anyhow!("{}", error)))),
        }
    }
}
