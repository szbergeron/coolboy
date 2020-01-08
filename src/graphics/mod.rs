extern crate sdl2;
extern crate rand;

use super::emulator::Emulator;

use rand::prelude::*;
use sdl2::Sdl;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::{Color, PixelFormatEnum};

const PIXEL_SIZE: u32 = 5;
const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

pub struct Screen {
    pub canvas: Canvas<Window>,
    // texture: Texture,
    i: u16,
    buffer: Vec<u32>,
    rng: ThreadRng, // TEMP: random pixels
}

impl Screen {
    pub fn new(sdl: &Sdl) -> Result<Self, String>{
        let video = sdl.video()?;
        let window = video.window("coolboy v1.0", WIDTH * PIXEL_SIZE, HEIGHT * PIXEL_SIZE)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        // let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT).map_err(|e| e.to_string())?;


        let screen = Screen { 
            canvas: canvas,
            i: 0,
            // texture: texture,
            buffer: vec![0; 160 * 144],
            rng: rand::thread_rng(),
        };

        Ok(screen)
    }

    pub fn update_buffer(&self, emu: &Emulator) {
        
    }

    pub fn draw(&mut self) {
        self.i = (self.i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(self.i as u8, 64, (255 - self.i) as u8));
        println!("changing");
        self.canvas.clear();
        self.canvas.present();
    }
}