use crate::display::Display;
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

    pub fn render(&mut self) {
        self.display.draw()
    }
}
