use std::fs;
use std::io::prelude::*;

enum MirrorMode {
  Vertical,
  Horizontal,
}

pub struct Cartridge {
  trainer_present: bool,
  prg_rom_data: Vec<u8>,
  chr_rom_data: Vec<u8>,
  mirror_mode: MirrorMode,
  mapper: u32,
}

impl Cartridge {
  pub fn load(path: &str) -> Cartridge {
    let data = fs::read(path).expect("Failed to read file");
    let header = &data[..16];

    let valid_ines = header[..4] == [0x4E, 0x45, 0x53, 0x1A];

    println!("Valid iNes file: {}", valid_ines);

    // false = horizontal, true = vertical mirror
    let char_mirror = mapMirrorMode(header[6] & 0b00000001);
    let trainer_present = (header[6] & 0b00000100) >> 2 == 1;
    let mapper = (header[7] & 0b11110000) + ((header[6] & 0b11110000) >> 4);
    let mut char_mirror_text = String::new();
    
    println!("Trainer present: {}", trainer_present);
    println!("Mapper: {}", mapper);
    println!("Character mirror mode: {}", char_mirror_text);
    println!("Size of PRG ROM: {} x 16k = {}k", header[4], header[4] * 16);
    println!("Size of CHR ROM: {} x 8192 = {} bytes", header[5], header[5] as usize * 8192);
  }
}

fn decodeHeader(header: &[u8]) {
  



}

pub fn print_ines_header(path: &str) {
  let data = fs::read(path).expect("Failed to read file");
  println!("{:X?}", data);
  println!("Total size = {}", data.len());
}

fn mapMirrorMode(mode_num: u8) -> Option<MirrorMode> {
  let mode = match mode_num {
    0 => return Some(MirrorMode::Horizontal),
    1 => return Some(MirrorMode::Vertical),
    _ => return None
  };
}