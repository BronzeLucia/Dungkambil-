use std::str;

const NES_HEADER_SIZE: usize = 0x0010;
const PROGRAM_ROM_SIZE: usize = 0x4000;
const CHARACTER_ROM_SIZE: usize = 0x2000;

pub struct Cassette {
    pub is_horizontal_mirror: bool,
    pub character_ram: Vec<u8>,
    pub program_rom: Vec<u8>,
    pub mapper: u8,
}

pub fn parse(buf: &mut [u8]) -> Cassette {
    let name = buf[0..3].to_vec();
    let ines = str::from_utf8(&name).unwrap();
    if ines != "NES" {
        panic!("Invalid *.nes file.")
    };
    let program_rom_pag