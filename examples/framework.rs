use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine::framework::*;

pub fn main() {
    let engine = wolf_engine::framework::init().build().unwrap();
    wolf_engine::framework::run(engine, MyGame::new())
}

pub struct MyGame {
    pixels: Option<Pixels>,
}

impl MyGame {
    pub fn new() -> Self {
        Self { pixels: None }
    }
}

impl Game for MyGame {
    fn setup(&mut self, context: &mut Context) {
        self.pixels = Some({
            let window = context.window();
            let (width, height) = window.size();
            let surface_texture = SurfaceTexture::new(width, height, &window);
            let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
            pixels.clear_color(Color::RED);
            pixels
        });
    }

    fn update(&mut self, _context: &mut Context) {}

    fn render(&mut self, _context: &mut Context) {
        if let Some(pixels) = self.pixels.as_ref() {
            pixels.render().unwrap();
        }
    }

    fn resized(&mut self, _context: &mut Context, new_size: (u32, u32)) {
        if let Some(pixels) = self.pixels.as_mut() {
            let (width, height) = new_size;
            pixels.resize_buffer(width, height).unwrap();
            pixels.resize_surface(width, height).unwrap();
        }
    }
}
