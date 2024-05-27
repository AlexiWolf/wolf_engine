use wolf_engine::window::WindowSettings;

pub fn main() {
    let (event_queue, context) = wolf_engine::window::init();
    let window = context.create_window(WindowSettings::default());

    loop {
        while let Some(event) = event_queue.next_event() {

        }
    }
}
