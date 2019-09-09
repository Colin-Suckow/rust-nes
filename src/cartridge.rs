use std::fs::File;
use std::io::prelude::*;

struct Cartridge {
  trainer_present: bool,
  prg_rom_pages: u32,
  chr_rom_pages: u32,
  prg_rom_data: Vec<u8>,
  chr_rom_data: Vec<u8>,
}

pub fn print_ines_header(path: &str) {
  let file = File::open(path);
  let mut contents: Vec<u8> = vec![0; 16];
  file
    .unwrap()
    .read_exact(&mut contents)
    .expect("ines header read fail");
  println!("{:X?}", contents);
}
