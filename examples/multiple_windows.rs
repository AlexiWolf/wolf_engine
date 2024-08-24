use std::collections::HashMap;

use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::window::event::Event;
use wolf_engine_window::{event::WindowEvent, Window, WindowSettings};

fn main() {
    let (event_loop, context) = wolf_engine::window::init().build().unwrap();
    let mut pixels = HashMap::new();
    let mut windows = HashMap::new();
    let window_settings = WindowSettings::default().with_resizable(false);
    let window_a_id = context.create_window(
        window_settings
            .clone()
            .with_title("Window A")
            .with_size((1280, 720)),
    );
    let _window_b_id = context.create_window(
        window_settings
            .clone()
            .with_title("Window B")
            .with_size((800, 600)),
    );

    event_loop.run(|event| match event {
        Event::Started => println!("Hello, world!"),
        Event::EventsCleared => windows.values().for_each(|window: &Window| window.redraw()),
        Event::WindowEvent(window_id, event) => match event {
            WindowEvent::Created(window_result) => {
                let window = window_result.expect("Window creation succeeded");
                let canvas = if window_id == window_a_id {
                    create_canvas(&window, Color::RED)
                } else {
                    create_canvas(&window, Color::BLUE)
                };
                windows.insert(window_id, window);
                pixels.insert(window_id, canvas);
            }
            WindowEvent::RedrawRequested => {
                let pixels = pixels.get(&window_id).unwrap();
                pixels.render().unwrap();
            }
            WindowEvent::Resized(width, height) => {
                let pixels = pixels.get_mut(&window_id).unwrap();
                pixels.resize_buffer(width, height).unwrap();
                pixels.resize_surface(width, height).unwrap();
            }
            WindowEvent::Input(input) => {
                println!("Input into window, {:?}: {:?}", window_id, input)
            }
            WindowEvent::Closed => {
                windows.remove(&window_id);
                if windows.len() == 0 {
                    context.exit();
                }
            }
            _ => (),
        },
        Event::Exited => println!("Goodbye, World!"),
        _ => (),
    });
}

fn create_canvas(window: &Window, background_color: Color) -> Pixels {
    let (width, height) = window.size();
    let surface_texture = SurfaceTexture::new(width, height, window);
    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
    pixels.clear_color(background_color);
    pixels
}
