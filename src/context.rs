extern crate sdl2;

pub struct Context {
    ctx: sdl2::Sdl,
}

impl Context {
    pub fn new() -> Self {
        Context { ctx: sdl2::init().unwrap() }
    }

    pub fn as_raw(&self) -> &sdl2::Sdl {
        &self.ctx
    }
}
