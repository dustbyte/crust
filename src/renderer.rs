use crate::display::Display;
use crate::cpu::CPU;

pub struct Renderer<'a> {
    display: &'a mut Display,
    cpu: &'a CPU,
}

impl<'a> Renderer<'a> {
    fn new(display: &'a mut Display, cpu: &'a CPU) -> Self {
        Renderer { display: display, cpu: cpu }
    }

    fn render(&mut self) {

    }
}
