extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::context::Context;

pub struct Keyboard {
    event_pump: sdl2::EventPump,
}

#[repr(u16)]
pub enum KeyPad {
    Key0 = 1 << 0x00,
    Key1 = 1 << 0x01,
    Key2 = 1 << 0x02,
    Key3 = 1 << 0x03,
    Key4 = 1 << 0x04,
    Key5 = 1 << 0x05,
    Key6 = 1 << 0x06,
    Key7 = 1 << 0x07,
    Key8 = 1 << 0x08,
    Key9 = 1 << 0x09,
    KeyA = 1 << 0x0A,
    KeyB = 1 << 0x0B,
    KeyC = 1 << 0x0C,
    KeyD = 1 << 0x0D,
    KeyE = 1 << 0x0E,
    KeyF = 1 << 0x0F,
}

pub struct State {
    state: u16,
}

impl State {
    pub fn new() -> Self {
        Self { state: 0 }
    }

    pub fn add_key(&mut self, key: KeyPad) {
        self.state |= key as u16
    }

    pub fn has_key(&self, keypos: u8) -> bool {
        (self.state & (1 << keypos)) != 0
    }

    pub fn as_raw(&self) -> u16 {
        self.state
    }
}

impl Keyboard {
    pub fn new(context: &Context) -> Self {
        Self {
            event_pump: context.as_raw().event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self) -> Result<State, ()> {
        let mut state = State::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Err(()),
                _ => (),
            }
        }

        let keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            match key {
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
                _ => (),
            }
        }

        return Ok(state);
    }
}

#[cfg(test)]
mod keyboard_test {
    use super::*;

    #[test]
    fn test_has_key() {
        let mut state = State::new();

        state.add_key(KeyPad::Key0);
        assert!(state.has_key(0));

        state.add_key(KeyPad::KeyA);
        assert!(state.has_key(0xa));

        assert!(!state.has_key(0x9));
    }
}
