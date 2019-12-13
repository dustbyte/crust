use crate::MEMORY_SIZE;
use crate::rom::RomBuffer;
use crate::font::FONT;

pub struct CPU {
    i: u16,
    pc: u16,
    sp: u8,
    v: [u8; 0xf],

    delay: u8,
    sound: u8,

    stack: [u16; 0xf],
    ram: [u8; MEMORY_SIZE],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            i: 0,
            pc: 0,
            sp: 0,
            v: [0; 0xf],
            delay: 0,
            sound: 0,
            stack: [0; 0xf],
            ram: [0; MEMORY_SIZE]
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
}
