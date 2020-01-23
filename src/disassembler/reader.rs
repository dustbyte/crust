use std::iter::FromIterator;

use crust::rom::RomBuffer;
use crust::ROM_SIZE;

pub struct RomReader<'a> {
    cur: usize,
    rom: &'a RomBuffer,
}

impl<'a> RomReader<'a> {
    pub fn new(rom: &'a RomBuffer) -> Self {
        Self { cur: 0, rom: rom }
    }
}

impl<'a> Iterator for RomReader<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur <= ROM_SIZE {
            let value = self.rom[self.cur];
            self.cur += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod reader_test {
    use super::*;

    #[test]
    fn test_new_rom_reader() {
        let rom: RomBuffer = [0; ROM_SIZE];
        let reader = RomReader::new(&rom);

        assert_eq!(reader.cur, 0);
    }

    #[test]
    fn test_iterate_rom() {
        let mut rom: RomBuffer = [0; ROM_SIZE];

        for (i, &elem) in [0x01, 0x02, 0x03, 0x04].iter().enumerate() {
            rom[i as usize] = elem;
        }

        let mut reader = RomReader::new(&rom);

        assert_eq!(reader.next(), Some(1));
        assert_eq!(reader.next(), Some(2));
        assert_eq!(reader.next(), Some(3));
        assert_eq!(reader.next(), Some(4));
    }
}
