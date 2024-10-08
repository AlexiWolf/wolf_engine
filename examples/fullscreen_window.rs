use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine_input::{keyboard::KeyCode, ButtonState, Input};
use wolf_engine_window::{
    event::{Event, WindowEvent},
    FullscreenMode, Window, WindowSettings,
};

fn main() {
    let (event_loop, context) = wolf_engine::window::init().build().unwrap();

    let mut pixels = None;
    let mut window: Option<Window> = None;

    event_loop.run(|event| match event {
        Event::Started => {
            println!("Hello, world!");
            context.create_window(
                WindowSettings::default()
                    .with_title("Wolf Engine - Window Example")
                    .with_fullscreen_mode(FullscreenMode::Borderless),
            );
        }
        Event::EventsCleared => {
            if let Some(window) = window.as_ref() {
                window.redraw();
            }
        }
        Event::WindowEvent(_window_id, event) => match event {
            WindowEvent::Created(window_result) => {
                window = Some(window_result.expect("Window creation succeeded"));
                pixels = Some({
                    let window = window.as_ref().unwrap();
                    let (width, height) = window.size();
                    let surface_texture = SurfaceTexture::new(width, height, window);
                    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
                    pixels.clear_color(Color::RED);
                    pixels
                });
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = pixels.as_ref() {
                    pixels.render().unwrap();
                }
            }
            WindowEvent::Resized(width, height) => {
                if let Some(pixels) = &mut pixels {
                    pixels.resize_buffer(width, height).unwrap();
                    pixels.resize_surface(width, height).unwrap();
                }
            }
            WindowEvent::Input(Input::Keyboard {
                state: ButtonState::Up,
                keycode: Some(KeyCode::Space),
                ..
            }) => {
                let window = window.as_ref().unwrap();
                if window.fullscreen_mode().is_some() {
                    window.set_fullscreen_mode(None);
                } else {
                    window.set_fullscreen_mode(Some(FullscreenMode::Borderless));
                }
            }
            WindowEvent::Closed => context.exit(),
            _ => (),
        },
        Event::Exited => println!("Goodbye, World!"),
        _ => (),
    });
}
