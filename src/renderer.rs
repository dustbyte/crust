use crate::display::{Display, DisplayColor};
use crate::cpu::CPU;

pub struct Renderer<'a> {
    display: &'a mut Display,
    cpu: &'a CPU,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a mut Display, cpu: &'a CPU) -> Self {
        Renderer { display: display, cpu: cpu }
    }

    pub fn reset(&mut self) {
        self.display.clear_screen();
    }

    fn get_color(pixel: u8) -> DisplayColor {
        if pixel == 0 {
            DisplayColor::Black
        } else {
            DisplayColor::White
        }
    }

    pub fn render(&mut self) {
        for (j, &line) in self.cpu.get_vram().iter().enumerate() {
            for (i, &pixel) in line.iter().enumerate() {
                self.display.draw_pixel(i as i32, j as i32, Self::get_color(pixel))
            }
        }
        self.display.draw()
    }
}
