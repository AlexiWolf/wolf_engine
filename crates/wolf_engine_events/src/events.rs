//! Provides the basic events commonly used by the engine.

use crate::dynamic::DynamicEvent;

/// Indicates the engine has quit.
#[derive(DynamicEvent, Debug)]
pub struct Quit;

/// Indicates all other events have been cleared, and a new frame / tick is about to begin.
#[derive(DynamicEvent, Debug)]
pub struct EventsCleared;
