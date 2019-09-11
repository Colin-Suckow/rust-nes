use std::fs;
use std::io::prelude::*;

pub struct Cartridge {
  trainer_present: bool,
  prg_rom_pages: u32,
  chr_rom_pages: u32,
  prg_rom_data: Vec<u8>,
  chr_rom_data: Vec<u8>,
}

impl Cartridge {
  pub fn load(path: &str) {
    let data = fs::read(path).expect("Failed to read file");
    decodeHeader(&data[..16]);
    
  }
}

fn decodeHeader(header: &[u8]) {
  let valid_ines = header[..4] == [0x4E, 0x45, 0x53, 0x1A];
  println!("Valid iNes file: {}", valid_ines);

  // false = horizontal, true = vertical mirror
  let char_mirror = header[6] & 0b00000001 == 1;
  let trainer_present = (header[6] & 0b00000100) >> 2 == 1;
  let mapper = (header[7] & 0b11110000) + ((header[6] & 0b11110000) >> 4);
  let mut char_mirror_text = String::new();

  if char_mirror {
    char_mirror_text = String::from("Vertical");
  } else {
    char_mirror_text = String::from("Horizontal");
  }
  
  println!("Trainer present: {}", trainer_present);
  println!("Mapper: {}", mapper);
  println!("Character mirror mode: {}", char_mirror_text);
}

pub fn print_ines_header(path: &str) {
  let data = fs::read(path).expect("Failed to read file");
  println!("{:X?}", data);
  println!("Total size = {}", data.len());
}
