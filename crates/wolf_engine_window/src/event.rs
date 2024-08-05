use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use anyhow::anyhow;
use uuid::Uuid;
use winit::{
    error::EventLoopError,
    event::{Event as WinitEvent, WindowEvent as WinitWindowEvent},
    event_loop::ControlFlow,
    window::{Window as WinitWindow, WindowId},
};
use wolf_engine_input::{Input, ToInput};

use crate::{
    error::{OsError, UnsupportedError, WindowError},
    WindowContext, WindowSettings,
};

/// An event generated by the window system.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    Resumed,
    RedrawRequested(Uuid),
    Resized(Uuid, u32, u32),
    Closed(Uuid),
    Input(Option<Uuid>, Input),
    Exited,
}

pub(crate) type WinitEventLoop = winit::event_loop::EventLoop<()>;

/// Provides a way to configure the window system.
///
/// Create a new builder by calling [`init()`].
pub struct EventLoopBuilder {
    pub window_settings: WindowSettings,
}

impl EventLoopBuilder {
    pub(crate) fn new() -> Self {
        Self {
            window_settings: WindowSettings::default(),
        }
    }

    /// Initialize the window system.
    pub fn build(self) -> Result<EventLoop, WindowError> {
        match WinitEventLoop::with_user_event().build() {
            Ok(event_loop) => Ok(self.build_with_event_loop(event_loop)),
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

    #[allow(deprecated)]
    pub(crate) fn build_with_event_loop(self, event_loop: WinitEventLoop) -> EventLoop {
        EventLoop::new(event_loop)
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
    pub fn run<F: FnMut(WindowEvent, &WindowContext)>(self, mut event_handler: F) {
        let window_ids: Arc<RwLock<HashMap<WindowId, Uuid>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let windows: Arc<RwLock<HashMap<Uuid, Weak<WinitWindow>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let _ = self.event_loop.run(|event, event_loop| {
            let context = WindowContext::new(event_loop, window_ids.clone(), windows.clone());

            match event {
                WinitEvent::NewEvents(..) => {
                    event_loop.set_control_flow(ControlFlow::Poll);

                    windows
                        .read()
                        .expect("write-lock was acquired")
                        .iter()
                        .for_each(|(_, window)| {
                            if let Some(window) = window.upgrade() {
                                window.request_redraw();
                            }
                        });
                }
                WinitEvent::Resumed => (event_handler)(WindowEvent::Resumed, &context),
                WinitEvent::LoopExiting => (event_handler)(WindowEvent::Exited, &context),
                WinitEvent::DeviceEvent { event, .. } => {
                    if let Some(input) = event.to_input() {
                        (event_handler)(WindowEvent::Input(None, input), &context);
                    }
                }
                WinitEvent::WindowEvent {
                    window_id,
                    event: window_event,
                } => {
                    let uuid = window_ids
                        .read()
                        .expect("read-lock was acquired")
                        .get(&window_id)
                        .expect("the uuid was inserted on window creation")
                        .to_owned();
                    if let Some(input) = window_event.to_input() {
                        (event_handler)(WindowEvent::Input(Some(uuid), input), &context);
                    }
                    match window_event {
                        WinitWindowEvent::RedrawRequested => {
                            (event_handler)(WindowEvent::RedrawRequested(uuid), &context)
                        }
                        WinitWindowEvent::Resized(size) => (event_handler)(
                            WindowEvent::Resized(uuid, size.width, size.height),
                            &context,
                        ),
                        WinitWindowEvent::CloseRequested => {
                            (event_handler)(WindowEvent::Closed(uuid), &context)
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });
    }
}
