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
const CPU_FREQUENCY: &'static str = "500";
const IO_FREQUENCY: &'static str = "60";

struct Counter {
    slice: u64,
    counter: time::Instant,
}

impl Counter {
    pub fn new(frequency: u64) -> Self {
        Self {
            slice: time::Duration::from_millis(1000 / frequency).as_millis() as u64,
            counter: time::Instant::now(),
        }
    }

    pub fn from_str(frequency: &str) -> Self {
        let int_value = frequency.to_string().parse::<u64>().unwrap();

        Self::new(int_value)
    }

    pub fn burnt_duration(&self) -> i128 {
        self.slice as i128 - self.counter.elapsed().as_millis() as i128
    }

    pub fn is_burnt(&self) -> bool {
        self.burnt_duration() <= 0
    }

    pub fn reset(&mut self) {
        self.counter = time::Instant::now()
    }

    pub fn duration(&self) -> time::Duration {
        self.counter.elapsed()
    }
}

pub fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("ROM")
            .help("path to the rom file")
            .required(true)
            )
        .arg(Arg::with_name("cpu_freq")
            .short("c")
            .long("cpu-freq")
            .value_name("cpu_freq")
            .help("Set the frequency of the CPU clock in Hz")
            )
        .arg(Arg::with_name("io_freq")
            .short("i")
            .long("io-freq")
            .value_name("io_freq")
            .help("Set the display and buzzer refresh rate in Hz")
            )
        .get_matches();

    let mut cpu_counter = Counter::from_str(matches.value_of("cpu_freq").unwrap_or(CPU_FREQUENCY));
    let mut io_counter = Counter::from_str(matches.value_of("io_freq").unwrap_or(IO_FREQUENCY));

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
        if io_counter.is_burnt() {
            renderer.render(cpu.get_vram());
            if cpu.beeping() {
                buzzer.play()
            } else {
                buzzer.pause()
            }
            io_counter.reset()
        }

        if cpu_counter.is_burnt() {
            cpu.tick(&state);
            cpu_counter.reset()
        } else {
            // This can be safely assume for a 500hz CPU block and 60HZ display refresh rate.
            thread::sleep(time::Duration::from_millis(cpu_counter.burnt_duration() as u64));
        }
    }
}
