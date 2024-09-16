use wolf_engine_events::{dynamic::AnyEvent, mpsc::MpscEventReceiver};
use wolf_engine_window::{error::WindowError, event::Event};

pub fn run<E: FnMut(Event)>(
    event_receiver: MpscEventReceiver<AnyEvent>,
    event_handler: E,
) -> Result<(), WindowError> {
    Ok(())
}
