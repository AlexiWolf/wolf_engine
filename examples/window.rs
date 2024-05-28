use wolf_engine::prelude::*;
use wolf_engine::window::WindowSettings;

pub fn main() {
    let (mut event_queue, context) = wolf_engine::window::init().unwrap();
    let window = context.create_window(WindowSettings::default());

    loop {
        while let Some(event) = event_queue.next_event() {
            match event {
                _ => (),
            }
        }
    }
}
