use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use wolf_engine_framework::*;
use wolf_engine_input::keyboard::KeyCode;
use wolf_engine_input::ButtonState;
use wolf_engine_input::Input;

pub fn main() {
    let engine = wolf_engine::framework::init().build().unwrap();
    wolf_engine::framework::run(engine, MyGame::new())
}

pub struct MyGame {
    color: Color,
    pixels: Option<Pixels>,
}

impl MyGame {
    pub fn new() -> Game<Self, game_state::Unloaded> {
        Game::new(Self {
            pixels: None,
            color: Color::RED,
        })
    }
}

impl EventHandler for MyGame {
    fn setup(&mut self, context: &mut Context) {
        self.pixels = Some({
            let window = context.window();
            let (width, height) = window.size();
            let surface_texture = SurfaceTexture::new(width, height, &window);
            let pixels = Pixels::new(width, height, surface_texture).unwrap();
            pixels
        });
    }

    fn update(&mut self, _context: &mut Context) {}

    fn input(&mut self, _context: &mut Context, input: Input) {
        println!("Input: {:?}", input);
        match input {
            Input::Keyboard {
                state: ButtonState::Down,
                keycode: Some(KeyCode::Space),
                ..
            } => self.color = Color::BLUE,
            Input::Keyboard {
                state: ButtonState::Up,
                keycode: Some(KeyCode::Space),
                ..
            } => self.color = Color::RED,
            _ => (),
        }
    }

    fn render(&mut self, _context: &mut Context) {
        if let Some(pixels) = self.pixels.as_mut() {
            pixels.clear_color(self.color);
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
