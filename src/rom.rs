use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::ROM_SIZE;

pub type RomBuffer = [u8; ROM_SIZE];

pub fn load_rom(path: &str) -> io::Result<RomBuffer> {
    let mut handle = File::open(path)?;
    let mut buffer: RomBuffer = [0; ROM_SIZE];

    handle.read(&mut buffer)?;

    Ok(buffer)
}

pub struct Reader<'a> {
    cur: usize,
    rom: &'a RomBuffer,
}

impl<'a> Reader<'a> {
    pub fn new(rom: &'a RomBuffer) -> Self {
        Self { cur: 0, rom: rom }
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < ROM_SIZE {
            let value = self.rom[self.cur];
            self.cur += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod rom_test {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_load_rom() {
        let mut handle = NamedTempFile::new().unwrap();

        handle.write_all(&[0xde, 0xad, 0xbe, 0xef]).unwrap();

        let buffer = load_rom(handle.path().to_str().unwrap()).unwrap();

        assert_eq!(buffer[0..4], [0xde, 0xad, 0xbe, 0xef])
    }

    #[test]
    fn test_new_rom_reader() {
        let rom: RomBuffer = [0; ROM_SIZE];
        let reader = Reader::new(&rom);

        assert_eq!(reader.cur, 0);
    }

    #[test]
    fn test_iterate_rom() {
        let mut rom: RomBuffer = [0; ROM_SIZE];

        for (i, &elem) in [0x01, 0x02, 0x03, 0x04].iter().enumerate() {
            rom[i as usize] = elem;
        }

        let mut reader = Reader::new(&rom);

        assert_eq!(reader.next(), Some(1));
        assert_eq!(reader.next(), Some(2));
        assert_eq!(reader.next(), Some(3));
        assert_eq!(reader.next(), Some(4));
    }
}
