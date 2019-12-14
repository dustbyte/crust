use crate::MEMORY_SIZE;
use crate::WIDTH;
use crate::HEIGHT;
use crate::rom::RomBuffer;
use crate::font::FONT;
use crate::keyboard::{State, KeyPad};

const INSTRUCTION_LENGTH: u16 = 2;
const GENERAL_PURPOSE_REGISTERS: usize = 0xf;
const STACK_SIZE: usize = 0xf;

pub type VRAM = [[u8; WIDTH]; HEIGHT];

pub struct CPU {
    i: u16,
    pc: u16,
    sp: u8,
    v: [u8; GENERAL_PURPOSE_REGISTERS],

    dt: u8,
    st: u8,

    stack: [u16; STACK_SIZE],
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
        self.run_code()
    }

    pub fn run_code(&mut self) {
        let instruction = self.fetch_instruction();
        self.pc += INSTRUCTION_LENGTH;
    }

    fn fetch_instruction(&self) -> u16 {
        ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1]) as u16
    }
}

#[cfg(test)]
mod cpu_test {
    use crate::ROM_SIZE;
    use super::*;

    #[test]
    fn test_new_cpu() {
        let cpu = CPU::new();

        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.v, [0; GENERAL_PURPOSE_REGISTERS]);
        assert_eq!(cpu.dt, 0);
        assert_eq!(cpu.st, 0);
        assert_eq!(cpu.stack, [0; STACK_SIZE]);
    }

    #[test]
    fn test_init_cpu() {
        let mut rom: RomBuffer = [0; ROM_SIZE];
        rom[0] = 0x42;
        let cpu = CPU::init(&rom);

        assert_eq!(cpu.pc, 0x200);
        assert_eq!(cpu.ram[0..5], FONT[0..5]);
        assert_eq!(cpu.ram[cpu.pc as usize], 0x42);
    }

    #[test]
    fn test_load_font() {
        let mut cpu = CPU::new();

        cpu.load_font();
        assert_eq!(cpu.ram[0..5], FONT[0..5]);
    }

    #[test]
    fn test_load_rom() {
        let mut rom: RomBuffer = [0; ROM_SIZE];
        let mut cpu = CPU::new();

        cpu.pc = 0x21;
        rom[0] = 0x42;

        cpu.load_rom(&rom);
        assert_eq!(cpu.ram[0x21], rom[0]);
    }

    #[test]
    fn test_get_ram() {
        let cpu = CPU::new();

        assert_eq!(cpu.get_ram() as *const [u8], &cpu.ram as *const [u8]);
    }

    #[test]
    fn test_get_vram() {
        let cpu = CPU::new();

        assert_eq!(cpu.get_vram() as *const VRAM, &cpu.vram as *const VRAM);
    }

    #[test]
    fn test_beeping() {
        let mut cpu = CPU::new();

        cpu.st = 128;

        assert!(cpu.beeping())
    }

    #[test]
    fn test_tick() {
        let mut rom: RomBuffer = [0; ROM_SIZE];
        let mut cpu = CPU::init(&rom);

        cpu.run_code();
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_fetch_instruction() {
        let mut rom: RomBuffer = [0; ROM_SIZE];

        for (i, &elem) in [0x01, 0x02, 0x03, 0x04].iter().enumerate() {
            rom[i as usize] = elem;
        }

        let mut cpu = CPU::init(&rom);

        let instruction = cpu.fetch_instruction();
        assert_eq!(instruction, 0x0102);
        cpu.pc += INSTRUCTION_LENGTH;
        let instruction = cpu.fetch_instruction();
        assert_eq!(instruction, 0x0304);
    }
}
