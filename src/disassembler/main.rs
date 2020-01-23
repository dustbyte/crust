extern crate clap;
mod reader;

use clap::{App, Arg, ArgMatches};

use crust::rom::load_rom;

use reader::RomReader;

fn run(matches: &ArgMatches) -> Result<(), String> {
    let rom_path = matches.value_of("ROM").unwrap();
    let rom = match load_rom(rom_path) {
        Ok(rom) => rom,
        Err(error) => return Err(error.to_string()),
    };
    let _reader = RomReader::new(&rom);
    Ok(())
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Chip-8 disassembler")
        .arg(
            Arg::with_name("ROM")
                .help("path to the rom file")
                .required(true),
        )
        .get_matches();

    if let Err(error) = run(&matches) {
        println!("Error: {}", error)
    }
}
