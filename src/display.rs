extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::context::Context;
use crate::{WIDTH, HEIGHT, UPSCALE};

const FACTOR: i32 = UPSCALE as i32;

pub enum DisplayColor {
    Black,
    White,
    RGB(u8, u8, u8),
}

impl DisplayColor {
    fn to_sdl_color(self) -> sdl2::pixels::Color {
        match self {
            DisplayColor::Black => return Color::RGB(0, 0, 0),
            DisplayColor::White => return Color::RGB(255, 255, 255),
            DisplayColor::RGB(r, g, b) => return Color::RGB(r, g, b),
        } 
    }
}

pub struct Display {
    canvas: sdl2::render::WindowCanvas,
}

impl Display {
    pub fn new(ctx: &Context) -> Self {
        let window = ctx.as_raw().video().unwrap()
            .window("Crust emulator", (WIDTH * UPSCALE) as u32, (HEIGHT * UPSCALE) as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .build()
            .unwrap();

        Self {
            canvas: canvas,
        }
    }

    pub fn set_color(&mut self, color: DisplayColor) {
        self.canvas.set_draw_color(DisplayColor::to_sdl_color(color))
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: DisplayColor) {
        self.set_color(color);
        self.canvas.fill_rect(Rect::new(x * FACTOR, y * FACTOR, FACTOR as u32, FACTOR as u32));
    }

    pub fn reset_screen(&mut self, r: u8, g: u8, b:u8) {
        self.set_color(DisplayColor::Black);
        self.canvas.clear();
    }

    pub fn clear_screen(&mut self) {
        self.reset_screen(0, 0, 0)
    }

    pub fn draw(&mut self) {
        self.canvas.present()
    }
}
