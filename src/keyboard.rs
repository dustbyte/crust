extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::display::Display;

pub struct Keyboard {
    event_pump: sdl2::EventPump,
}

//pub const Key0: u16 = (0x01 << 0x00);
//pub const Key1: u16 = (0x01 << 0x01);
//pub const Key2: u16 = (0x01 << 0x02);
//pub const Key3: u16 = (0x01 << 0x03);
//pub const Key4: u16 = (0x01 << 0x04);
//pub const Key5: u16 = (0x01 << 0x05);
//pub const Key6: u16 = (0x01 << 0x06);
//pub const Key7: u16 = (0x01 << 0x07);
//pub const Key8: u16 = (0x01 << 0x08);
//pub const Key9: u16 = (0x01 << 0x09);
//pub const KeyA: u16 = (0x01 << 0x0A);
//pub const KeyB: u16 = (0x01 << 0x0B);
//pub const KeyC: u16 = (0x01 << 0x0C);
//pub const KeyD: u16 = (0x01 << 0x0D);
//pub const KeyE: u16 = (0x01 << 0x0E);
//pub const KeyF: u16 = (0x01 << 0x0F);

#[repr(u16)]
pub enum KeyPad {
    Key0 = (0x01 << 0x00),
    Key1 = (0x01 << 0x01),
    Key2 = (0x01 << 0x02),
    Key3 = (0x01 << 0x03),
    Key4 = (0x01 << 0x04),
    Key5 = (0x01 << 0x05),
    Key6 = (0x01 << 0x06),
    Key7 = (0x01 << 0x07),
    Key8 = (0x01 << 0x08),
    Key9 = (0x01 << 0x09),
    KeyA = (0x01 << 0x0A),
    KeyB = (0x01 << 0x0B),
    KeyC = (0x01 << 0x0C),
    KeyD = (0x01 << 0x0D),
    KeyE = (0x01 << 0x0E),
    KeyF = (0x01 << 0x0F),
}

pub struct State {
    state: u16,
}

impl State {
    pub fn new() -> Self {
        State { state: 0 }
    }

    pub fn add_key(&mut self, key: KeyPad) {
        self.state |= key as u16
    }

    pub fn has_key(&self, key: KeyPad) -> bool {
        self.state & key as u16 != 0
    }

    pub fn as_raw(&self) -> u16 {
        self.state
    }
}

impl Keyboard {
    pub fn new(display: &Display) -> Keyboard {
        Keyboard {
            event_pump: display.get_context().event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self) -> Result<State, ()> {
        let mut state = State::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Err(()),
                Event::KeyDown { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Num1 => state.add_key(KeyPad::Key1),
                        Keycode::Num2 => state.add_key(KeyPad::Key2),
                        Keycode::Num3 => state.add_key(KeyPad::Key3),
                        Keycode::Num4 => state.add_key(KeyPad::KeyC),
                        Keycode::Q => state.add_key(KeyPad::Key4),
                        Keycode::W => state.add_key(KeyPad::Key5),
                        Keycode::E => state.add_key(KeyPad::Key6),
                        Keycode::R => state.add_key(KeyPad::KeyD),
                        Keycode::A => state.add_key(KeyPad::Key7),
                        Keycode::S => state.add_key(KeyPad::Key8),
                        Keycode::D => state.add_key(KeyPad::Key9),
                        Keycode::F => state.add_key(KeyPad::KeyE),
                        Keycode::Z => state.add_key(KeyPad::KeyA),
                        Keycode::X => state.add_key(KeyPad::Key0),
                        Keycode::C => state.add_key(KeyPad::KeyB),
                        Keycode::V => state.add_key(KeyPad::KeyF),
                        Keycode::Escape => return Err(()),
                        _ => ()
                    }
                }
                _ => (),
            }
        }

        return Ok(state)
    }
}
