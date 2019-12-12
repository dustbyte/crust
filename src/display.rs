extern crate sdl2;

use sdl2::pixels::Color;

pub struct Display {
    ctx: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,
}

impl Display {
    pub fn new() -> Display {
        let ctx = sdl2::init().unwrap();
        let window = ctx.video().unwrap()
            .window("Crust emulator", 630, 310)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .build()
            .unwrap();

        Display {
            ctx: ctx,
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

    pub fn render(&mut self) {
        self.canvas.present()
    }

    pub fn get_context(&self) -> &sdl2::Sdl {
        &self.ctx
    }
}
