pub fn main() {
    let window_context = wolf_engine::window::init()
        .with_title("Wolf Engine - Window Example")
        .with_size((800, 600))
        .resizable(false)
        .build();

    window_context.run(|event, window, _| {
        WindowEvent::Resume => (),
        WindowEvent::Render => (),
        WindowEvent::Closed => (),
        _ => (),
    });
}
