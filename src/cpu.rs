use rand;

use crate::font::FONT;
use crate::keyboard::State;
use crate::rom::RomBuffer;
use crate::HEIGHT;
use crate::MEMORY_SIZE;
use crate::WIDTH;

const INSTRUCTION_LENGTH: u16 = 2;
const GENERAL_PURPOSE_REGISTERS: usize = 0x10;
const STACK_SIZE: usize = 0x10;

pub type VRAM = [[bool; WIDTH]; HEIGHT];

pub struct CPU {
    i: usize,
    pc: u16,
    sp: usize,
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
            v: [0; GENERAL_PURPOSE_REGISTERS],
            dt: 0,
            st: 0,
            stack: [0; STACK_SIZE],
            ram: [0; MEMORY_SIZE],
            vram: [[false; WIDTH]; HEIGHT],
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

    pub fn get_vram(&self) -> &VRAM {
        &self.vram
    }

    pub fn beeping(&self) -> bool {
        self.st > 0
    }

    pub fn decrement_delay(&mut self) {
        if self.dt > 0 {
            self.dt -= 1
        }
    }

    pub fn tick(&mut self, input: &State) {
        let instruction = self.fetch_instruction();

        let nibbles = (
            (instruction & 0xF000) >> 12,
            (instruction & 0x0F00) >> 8,
            (instruction & 0x00F0) >> 4,
            (instruction & 0x000F),
        );

        let x: usize = nibbles.1 as usize;
        let y: usize = nibbles.2 as usize;
        let n: usize = nibbles.3 as usize;

        let kk: u8 = (instruction & 0x00FF) as u8;
        let nnn: u16 = instruction & 0x0FFF;

        self.pc += INSTRUCTION_LENGTH;

        match nibbles {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                self.vram = [[false; WIDTH]; HEIGHT];
            }
            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            // JP addr
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            // CALL addr
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // SE Vx, byte
            (0x3, _, _, _) => {
                if self.v[x] == kk {
                    self.pc += INSTRUCTION_LENGTH;
                }
            }
            // SNE Vx, byte
            (0x4, _, _, _) => {
                if self.v[x] != kk {
                    self.pc += INSTRUCTION_LENGTH;
                }
            }
            // SE Vx, Vy
            (0x5, _, _, 0x0) => {
                if self.v[x] == self.v[y] {
                    self.pc += INSTRUCTION_LENGTH;
                }
            }
            // LD Vx, byte
            (0x6, _, _, _) => {
                self.v[x] = kk;
            }
            // ADD Vx, byte
            (0x7, _, _, _) => {
                let vx = self.v[x] as u16;
                let val = kk as u16;
                let result = vx + val;
                self.v[x] = (result & 0xff) as u8;
            }
            // LD Vx, Vy
            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
            }
            // OR Vx, Vy
            (0x8, _, _, 0x1) => {
                self.v[x] |= self.v[y];
            }
            // AND Vx, Vy
            (0x8, _, _, 0x2) => {
                self.v[x] &= self.v[y];
            }
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => {
                self.v[x] ^= self.v[y];
            }
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                let (result, overflowed) = self.v[x].overflowing_add(self.v[y]);

                self.v[0xf] = if overflowed { 1 } else { 0 };

                self.v[x] = result;
            }
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                let (result, overflowed) = self.v[x].overflowing_sub(self.v[y]);

                self.v[0xf] = if overflowed { 1 } else { 0 };

                self.v[x] = result;
            }
            // SHR Vx {, Vy}
            (0x8, _, _, 0x6) => {
                self.v[0xf] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                let (result, overflowed) = self.v[y].overflowing_sub(self.v[x]);

                self.v[0xf] = if overflowed { 1 } else { 0 };

                self.v[x] = result;
            }
            // SHL Vx {, Vy}
            (0x8, _, _, 0xE) => {
                self.v[0xf] = self.v[x] & 0x1;
                self.v[x] <<= 1;
            }
            // SNE Vx, Vy
            (0x9, _, _, 0x00) => {
                if self.v[x] != self.v[y] {
                    self.pc += INSTRUCTION_LENGTH;
                }
            }
            // LD I, addr
            (0xA, _, _, _) => {
                self.i = nnn as usize;
            }
            // JP V0, addr
            (0xB, _, _, _) => {
                self.pc = nnn + (self.v[0] as u16);
            }
            // RND Vx, byte
            (0xC, _, _, _) => {
                self.v[x] = rand::random::<u8>() & kk;
            }
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                for idx in 0..n {
                    for bit in 0..8 {
                        let posx = (self.v[x] as usize + bit) % WIDTH;
                        let posy = (self.v[y] as usize + idx) % HEIGHT;
                        let cell = &mut self.vram[posy][posx];

                        *cell ^= (self.ram[(self.i + idx) as usize] << bit >> 7) != 0;
                    }
                }
            }
            // SKP Vx
            (0xE, _, 0x9, 0xE) => {
                if input.has_key(self.v[x]) {
                    self.pc += INSTRUCTION_LENGTH;
                }
            }
            // SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                if !input.has_key(self.v[x]) {
                    self.pc += INSTRUCTION_LENGTH;
                }
            }
            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.dt;
            }
            // LD Vx, K
            (0xF, _, 0x0, 0xA) => {
                self.pc -= INSTRUCTION_LENGTH;
                let raw_input = input.as_raw();

                for bit in 0x0..0x10 {
                    if (raw_input << bit >> 7) == 1 {
                        self.pc += INSTRUCTION_LENGTH;
                        self.v[x] = bit;
                        return;
                    }
                }
            }
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => {
                self.dt = self.v[x];
            }
            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => {
                self.st = self.v[x];
            }
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => {
                self.i += self.v[x] as usize;
            }
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => {
                self.i = self.v[x] as usize * 5;
            }
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => {
                self.ram[self.i] = self.v[x] / 100;
                self.ram[self.i + 1] = (self.v[x] % 100) / 10;
                self.ram[self.i + 2] = self.v[x] % 10;
            }
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => {
                for idx in 0x0..x + 1 {
                    self.ram[self.i + idx] = self.v[idx];
                }
            }
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => {
                for idx in 0x0..x + 1 {
                    self.v[idx] = self.ram[self.i + idx];
                }
            }
            (_, _, _, _) => {
                println!("Unknown instruction 0x{:x}", instruction);
            }
        }
    }

    #[allow(dead_code)]
    fn print_state(&self, instruction: u16) {
        println!("Instruction: 0x{:x}", instruction);
        for x in 0..0x10 {
            print!("V{:X} = 0x{:x}\n", x, self.v[x]);
        }
        println!(
            "i = 0x{:04x}\npc = 0x{:04x}\nsp = 0x{:x}\ndt = 0x{:x}\nst = 0x{:x}\n",
            self.i, self.pc, self.sp, self.dt, self.st
        );
    }

    #[allow(dead_code)]
    fn print_memory(&self, from: usize, length: usize) {
        for x in from..from + length {
            println!("{:04x}: {:02x}", x, self.ram[x]);
        }
    }

    fn fetch_instruction(&self) -> u16 {
        ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1]) as u16
    }
}

#[cfg(test)]
mod cpu_test {
    use super::*;
    use crate::ROM_SIZE;

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
    fn test_decrement_delay() {
        let mut cpu = CPU::new();

        cpu.dt = 128;
        cpu.decrement_delay();

        assert_eq!(cpu.dt, 127);

        for _ in 1..128 {
            cpu.decrement_delay()
        }

        assert_eq!(cpu.dt, 0);
        cpu.decrement_delay();
        assert_eq!(cpu.dt, 0);
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
