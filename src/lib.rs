pub mod decoder;
pub mod rom;

pub const MEMORY_SIZE: usize = 0x1000;
pub const ROM_SIZE: usize = MEMORY_SIZE - 0x200;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
