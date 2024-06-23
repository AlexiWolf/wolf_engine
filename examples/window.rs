use wolf_engine::window::WindowEvent;

fn main() {
    let window_context = wolf_engine::window::init()
        .with_title("Wolf Engine - Window Example")
        .with_size((800, 600))
        .with_resizable(false)
        .build();

    window_context.run(|event, window| match event {
        WindowEvent::Resume => (),
        WindowEvent::Render => (),
        WindowEvent::Closed => (),
        _ => (),
    });
}
