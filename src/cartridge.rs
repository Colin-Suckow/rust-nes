use crate::memory;
use std::fs;

#[derive(Debug)]
pub enum MirrorMode {
  Vertical,
  Horizontal,
}

pub struct Cartridge {
  pub trainer_present: bool,
  pub prg_rom_data: Vec<u8>,
  pub chr_rom_data: Vec<u8>,
  pub mirror_mode: MirrorMode,
  pub mapper: u32,
}

impl Cartridge {
  pub fn load(path: &str) -> Cartridge {
    let data = fs::read(path).expect("Failed to read file");
    let header = &data[..16];

    let valid_ines = header[..4] == [0x4E, 0x45, 0x53, 0x1A];
    if (!valid_ines) {
      panic!("File is not a valid nes rom!");
    }
    // false = horizontal, true = vertical mirror
    let char_mirror = mapMirrorMode(header[6] & 0b00000001).expect("Unsupported mirror mode!");
    let trainer_present = (header[6] & 0b00000100) >> 2 == 1;
    let mapper = (header[7] & 0b11110000) + ((header[6] & 0b11110000) >> 4);
    let mut char_mirror_text = String::new();

    let prg_start = 16 as usize;
    let prg_end = prg_start + (header[4] as usize * 16384) - 1;
    let chr_start = prg_end as usize;
    let chr_end = chr_start + (header[5] as usize * 8192) as usize;

    Cartridge {
      trainer_present: trainer_present,
      prg_rom_data: data[prg_start..prg_end].to_vec(),
      chr_rom_data: data[chr_start..chr_end].to_vec(),
      mirror_mode: char_mirror,
      mapper: mapper as u32,
    }
  }

  pub fn printStats(&self) {
    println!("Mapper: {}", self.mapper);
    println!("Character Mirroring: {:?}", self.mirror_mode);
    println!("Program ROM size: {} bytes", self.prg_rom_data.len());
    println!("Character ROM size: {} bytes", self.chr_rom_data.len());
  }
}

impl memory::AddressSpace for Cartridge {
  fn peek(&self, ptr: u16) -> Option<u8> {
    if ptr > self.prg_rom_data.len() as u16 {
      Some(self.prg_rom_data[ptr as usize])
    } else {
      Some(self.chr_rom_data[ptr as usize])
    }
  }

  fn poke(&mut self, ptr: u16, byte: u8) {
    if ptr > self.prg_rom_data.len() as u16 {
      self.prg_rom_data[ptr as usize] = byte;
    } else {
      self.chr_rom_data[ptr as usize] = byte;
    }
  }
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
    _ => return None,
  };
}
