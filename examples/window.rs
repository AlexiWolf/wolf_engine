use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use rand::Rng;
use wolf_engine_events::event_loop::EventLoop;
use wolf_engine_window::{backend::WindowSystem, Window, WindowEvent, WindowSettings};

fn main() {
    let mut rng = rand::thread_rng();
    let window_system = wolf_engine_winit::init().unwrap();
    let context = window_system.context();

    let window = context.create_window(
        WindowSettings::default()
            .with_title("Wolf Engine - Window Example")
            .with_size((800, 600)),
    );
    let mut canvas = Canvas::from(window);

    window_system.run(|event| {
        if let Some(window_event) = event.downcast_ref::<WindowEvent>() {
            match window_event {
                WindowEvent::Started => {
                    println!("Hello, world!");
                }
                WindowEvent::EventsCleared => {
                    canvas.window().redraw();
                }
                WindowEvent::WindowReady(_, _) => {
                    canvas.reconfigure();
                    canvas.pixels_mut().unwrap().enable_vsync(false);
                }
                WindowEvent::WindowRedrawRequested(_) => {
                    let (width, height) = canvas.window.size();
                    for x in 0..width {
                        for y in 0..height {
                            if rng.gen() {
                                canvas.draw_pixel(x, y, [0xff, 0xff, 0xff, 0xff]);
                            } else {
                                canvas.draw_pixel(x, y, [0x00, 0x00, 0x00, 0xff]);
                            }
                        }
                    }
                    canvas.render();
                }
                WindowEvent::WindowResized(_, _, _) => {
                    canvas.reconfigure();
                }
                WindowEvent::Input(uuid, input) => {
                    println!("Input into window ({:?}): {:?}", uuid, input)
                }
                WindowEvent::WindowClosed(_) => context.exit(),
                WindowEvent::Exited => println!("Goodbye, World!"),
                _ => (),
            }
        }
    });
}

pub struct Canvas {
    pixels: Option<Pixels>,
    window: Window,
}

impl From<Window> for Canvas {
    fn from(window: Window) -> Self {
        Self {
            pixels: None,
            window,
        }
    }
}

impl Canvas {
    pub fn reconfigure(&mut self) {
        if self.window.handle().is_none() {
            return;
        }
        let (width, height) = self.window.size();
        if self.pixels.is_none() {
            let handle = self.window.handle().unwrap();
            let surface_texture = SurfaceTexture::new(width, height, &handle);
            let pixels = Pixels::new(width, height, surface_texture).unwrap();
            self.pixels = Some(pixels);
        }

        self.pixels
            .as_mut()
            .unwrap()
            .resize_surface(width, height)
            .unwrap();
        self.pixels
            .as_mut()
            .unwrap()
            .resize_buffer(width, height)
            .unwrap();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn pixels(&self) -> Option<&Pixels> {
        if let Some(pixels) = &self.pixels {
            Some(pixels)
        } else {
            None
        }
    }

    pub fn pixels_mut(&mut self) -> Option<&mut Pixels> {
        if let Some(pixels) = &mut self.pixels {
            Some(pixels)
        } else {
            None
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if let Some(pixels) = &mut self.pixels {
            let (width, _) = self.window.size();
            let index = x as usize + y as usize * width as usize;
            write_pixel(color, index, pixels.frame_mut());
        }
    }

    pub fn render(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            pixels.render().unwrap();
        }
    }
}

pub fn write_pixel(color: [u8; 4], index: usize, buffer: &mut [u8]) {
    let pixel_index = index * 4;
    if pixel_index > buffer.len() {
        panic!(
            "Index out of bounds.  Buffer size is {}, but index was {}",
            buffer.len(),
            pixel_index
        )
    }
    for (channel_i, value) in color.iter().enumerate() {
        buffer[pixel_index + channel_i] = *value;
    }
}
