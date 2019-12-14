extern crate clap;
extern crate hexdump;
mod cpu;
mod rom;
mod font;
mod context;
mod display;
mod buzzer;
mod keyboard;
mod renderer;

use std::{thread, time};

use clap::{Arg, App};

use cpu::CPU;
use rom::*;
use context::Context;
use display::Display;
use buzzer::Buzzer;
use keyboard::*;
use renderer::Renderer;

const MEMORY_SIZE: usize = 0x1000;
const ROM_SIZE: usize = MEMORY_SIZE - 0x200;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const UPSCALE: usize= 10;

pub fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("ROM")
            .help("path to the rom file")
            .required(true)
            )
        .get_matches();

    let rom_path = matches.value_of("ROM").unwrap();
    let rom = load_rom(rom_path).unwrap();
    let mut cpu = CPU::init(&rom);

    let context = Context::new();
    let mut display = Display::new(&context);
    let mut buzzer = Buzzer::new(&context);
    let mut keyboard = Keyboard::new(&context);
    let mut renderer = Renderer::new(&mut display);

    renderer.reset();
    while let Ok(state) = keyboard.poll() {
        cpu.tick(&state);

        renderer.render(cpu.get_vram());
        if cpu.beeping() {
            buzzer.play()
        } else {
            buzzer.pause()
        }

        // tmp 500hz
        thread::sleep(time::Duration::from_millis(2));
    }
}
