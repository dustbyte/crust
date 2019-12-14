use crate::MEMORY_SIZE;
use crate::WIDTH;
use crate::HEIGHT;
use crate::rom::RomBuffer;
use crate::font::FONT;
use crate::keyboard::{State, KeyPad};

pub type VRAM = [[u8; WIDTH]; HEIGHT];

pub struct CPU {
    i: u16,
    pc: u16,
    sp: u8,
    v: [u8; 0xf],

    dt: u8,
    st: u8,

    stack: [u16; 0xf],
    ram: [u8; MEMORY_SIZE],
    vram: VRAM,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            i: 0,
            pc: 0,
            sp: 0,
            v: [0; 0xf],
            dt: 0,
            st: 0,
            stack: [0; 0xf],
            ram: [0; MEMORY_SIZE],
            vram: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn init(rom: &RomBuffer) -> CPU {
        let mut cpu = CPU::new();
        cpu.pc = 0x200;

        cpu.load_font();
        cpu.load_rom(&rom);

        cpu
    }

    fn load_font(&mut self) {
        for (i, &elem) in FONT.iter().enumerate() {
            self.ram[i] = elem
        }
    }

    fn load_rom(&mut self, rom: &RomBuffer) {
        for (i, &elem) in rom.iter().enumerate() {
            self.ram[self.pc as usize + i] = elem
        }
    }

    pub fn get_ram(&self) -> &[u8] {
        &self.ram
    }

    pub fn get_vram(&self) -> &VRAM {
        &self.vram
    }

    pub fn beeping(&self) -> bool {
        self.st > 0
    }

    pub fn tick(&mut self, input: &State) {

    }
}
