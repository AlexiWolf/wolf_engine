use std::collections::HashMap;

use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::window::event::WindowEvent;
use wolf_engine_window::{Uuid, Window, WindowContext, WindowSettings};

fn main() {
    let context = wolf_engine::window::init().build().unwrap();

    let mut windows = HashMap::new();
    let mut pixels = HashMap::new();

    context.run(|event, context| match event {
        WindowEvent::Resumed => {
            println!("Hello, world!");
            create_window(&context, Color::RED, &mut windows, &mut pixels);
            create_window(&context, Color::BLUE, &mut windows, &mut pixels);
        }
        WindowEvent::RedrawRequested(window_id) => {
            let pixels = pixels.get(&window_id).unwrap();
            pixels.render().unwrap();
        }
        WindowEvent::Resized(window_id, width, height) => {
            let pixels = pixels.get_mut(&window_id).unwrap();
            pixels.resize_buffer(width, height).unwrap();
            pixels.resize_surface(width, height).unwrap();
        }
        WindowEvent::Input(window_id, input) => {
            println!("Input into window, {:?}: {:?}", window_id, input)
        }
        WindowEvent::Closed(window_id) => {
            remove_window(&window_id, &mut windows, &mut pixels);
            if windows.len() == 0 {
                context.exit();
            }
        }
        WindowEvent::Exited => println!("Goodbye, World!"),
        _ => (),
    });
}

fn create_window(
    context: &WindowContext,
    background_color: Color,
    windows: &mut HashMap<Uuid, Window>,
    pixels: &mut HashMap<Uuid, Pixels>,
) {
    let window = context
        .create_window(
            WindowSettings::default()
                .with_title("Wolf Engine - Multi-Window Example")
                .with_size((800, 600))
                .with_resizable(false),
        )
        .expect("window creation succeeded");
    let pixels_instance = {
        let (width, height) = window.size();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
        pixels.clear_color(background_color);
        pixels
    };

    let window_id = window.id();
    windows.insert(window_id, window);
    pixels.insert(window_id, pixels_instance);
}

fn remove_window(
    uuid: &Uuid,
    windows: &mut HashMap<Uuid, Window>,
    pixels: &mut HashMap<Uuid, Pixels>,
) {
    pixels.remove(uuid);
    windows.remove(uuid);
}
