pub fn main() {
    let (event_loop, context) = wolf_engine::window::init();
    let window = context.create_window(WindowSettings::default());
}
