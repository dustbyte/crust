extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::display::Display;

pub struct Keyboard {
    event_pump: sdl2::EventPump,
}

const Key0: u16 = (0x01 << 0x00);
const Key1: u16 = (0x01 << 0x01);
const Key2: u16 = (0x01 << 0x02);
const Key3: u16 = (0x01 << 0x03);
const Key4: u16 = (0x01 << 0x04);
const Key5: u16 = (0x01 << 0x05);
const Key6: u16 = (0x01 << 0x06);
const Key7: u16 = (0x01 << 0x07);
const Key8: u16 = (0x01 << 0x08);
const Key9: u16 = (0x01 << 0x09);
const KeyA: u16 = (0x01 << 0x0A);
const KeyB: u16 = (0x01 << 0x0B);
const KeyC: u16 = (0x01 << 0x0C);
const KeyD: u16 = (0x01 << 0x0D);
const KeyE: u16 = (0x01 << 0x0E);
const KeyF: u16 = (0x01 << 0x0F);

pub type State = u16;

impl Keyboard {
    pub fn new(display: &Display) -> Keyboard {
        Keyboard {
            event_pump: display.get_context().event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self) -> Result<State, ()> {
        let mut state: State = 0;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Err(()),
                Event::KeyDown { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Num1 => state |= Key1,
                        Keycode::Num2 => state |= Key2,
                        Keycode::Num3 => state |= Key3,
                        Keycode::Num4 => state |= KeyC,
                        Keycode::Q => state |= Key4,
                        Keycode::W => state |= Key5,
                        Keycode::E => state |= Key6,
                        Keycode::R => state |= KeyD,
                        Keycode::A => state |= Key7,
                        Keycode::S => state |= Key8,
                        Keycode::D => state |= Key9,
                        Keycode::F => state |= KeyE,
                        Keycode::Z => state |= KeyA,
                        Keycode::X => state |= Key0,
                        Keycode::C => state |= KeyB,
                        Keycode::V => state |= KeyF,
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
