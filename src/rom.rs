use crate::ROM_SIZE;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub type RomBuffer = [u8; ROM_SIZE];

pub fn load_rom(path: &str) -> io::Result<RomBuffer> {
    let mut handle = File::open(path)?;
    let mut buffer: RomBuffer = [0; ROM_SIZE];

    handle.read(&mut buffer)?;

    Ok(buffer)
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
}
