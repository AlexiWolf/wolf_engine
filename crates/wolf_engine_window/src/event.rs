//! Provides an event system for the window crate.

use anyhow::anyhow;
use uuid::Uuid;
use winit::{
    error::EventLoopError,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::ControlFlow,
};
use wolf_engine_input::{Input, ToInput};

use crate::{
    error::{OsError, UnsupportedError, WindowError},
    WindowContext, WindowIdMap,
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
    RedrawRequested,
    Resized(u32, u32),
    Closed,
    Input(Input),
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
    pub fn build(self) -> Result<EventLoop, WindowError> {
        match WinitEventLoop::new() {
            Ok(event_loop) => Ok(EventLoop::new(event_loop)),
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
    event_loop: WinitEventLoop,
}

impl EventLoop {
    fn new(event_loop: WinitEventLoop) -> Self {
        Self { event_loop }
    }
}

impl EventLoop {
    /// Run the main-loop, passing events to the provided callback.
    #[allow(deprecated)]
    pub fn run<F: FnMut(Event, &WindowContext)>(self, mut event_handler: F) {
        let mut is_first_resume = true;
        let window_ids = WindowIdMap::new();
        let _ = self.event_loop.run(|event, event_loop| {
            let context = WindowContext::new(event_loop, window_ids.clone());
            match event {
                WinitEvent::AboutToWait => {
                    (event_handler)(Event::EventsCleared, &context);
                    event_loop.set_control_flow(ControlFlow::Poll);
                }
                WinitEvent::Resumed => {
                    if is_first_resume {
                        is_first_resume = false;
                        (event_handler)(Event::Started, &context);
                    }
                }
                WinitEvent::LoopExiting => (event_handler)(Event::Exited, &context),
                WinitEvent::DeviceEvent { event, .. } => {
                    if let Some(input) = event.to_input() {
                        (event_handler)(Event::Input(input), &context);
                    }
                }
                WinitEvent::WindowEvent {
                    window_id,
                    event: window_event,
                } => {
                    let uuid = match window_ids.uuid_of(window_id) {
                        Some(uuid) => uuid,
                        None => return,
                    };

                    if let Some(input) = window_event.to_input() {
                        (event_handler)(
                            Event::WindowEvent(uuid, WindowEvent::Input(input)),
                            &context,
                        );
                    }
                    match window_event {
                        WinitWindowEvent::RedrawRequested => (event_handler)(
                            Event::WindowEvent(uuid, WindowEvent::RedrawRequested),
                            &context,
                        ),
                        WinitWindowEvent::Resized(size) => (event_handler)(
                            Event::WindowEvent(uuid, WindowEvent::Resized(size.width, size.height)),
                            &context,
                        ),
                        WinitWindowEvent::CloseRequested => {
                            (event_handler)(Event::WindowEvent(uuid, WindowEvent::Closed), &context)
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });
    }
}
