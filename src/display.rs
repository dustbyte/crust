extern crate sdl2;

use sdl2::pixels::Color;

use crate::context::Context;

pub struct Display {
    canvas: sdl2::render::WindowCanvas,
}

impl Display {
    pub fn new(ctx: &Context) -> Self {
        let window = ctx.as_raw().video().unwrap()
            .window("Crust emulator", 630, 310)
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

    pub fn reset_screen(&mut self, r: u8, g: u8, b:u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.clear();
    }

    pub fn clear_screen(&mut self) {
        self.reset_screen(0, 0, 0)
    }

    pub fn draw(&mut self) {
        self.canvas.present()
    }
}
