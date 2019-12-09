extern crate clap;
extern crate hexdump;
mod cpu;
mod rom;
mod font;

use hexdump::hexdump;
use clap::{Arg, App};

use cpu::CPU;
use rom::*;

const MEMORY_SIZE: usize = 0x1000;
const ROM_SIZE: usize = MEMORY_SIZE - 0x200;

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
    let cpu = CPU::init(&rom);

    hexdump(&cpu.get_ram())
}
