use crate::memory;
use std::fs;

#[derive(Debug, Clone, Copy)]
pub enum MirrorMode {
    Vertical,
    Horizontal,
}

pub struct Cartridge {
    pub trainer_present: bool,
    pub prg_rom_data: Option<Vec<u8>>,
    pub chr_rom_data: Option<Vec<u8>>,
    pub mirror_mode: MirrorMode,
    pub mapper: u32,
}

impl Cartridge {
    pub fn load(data: Vec<u8>) -> Cartridge {
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
        let prg_end = prg_start + (header[4] as usize * 16384);
        let chr_start = prg_end as usize;
        let chr_end = chr_start + (header[5] as usize * 8192) as usize;

        // for val in data[prg_start..prg_end].to_vec() {
        //     print!("{:#X} ", val);
        // }

        Cartridge {
            trainer_present: trainer_present,
            prg_rom_data: Some(data[prg_start..prg_end].to_vec()),
            chr_rom_data: Some(data[chr_start..chr_end].to_vec()),
            mirror_mode: char_mirror,
            mapper: mapper as u32,
        }
    }

    pub fn take_program_data(&mut self) -> ProgramData {
        ProgramData {
            data: self.prg_rom_data.take().unwrap(),
        }
    }

    pub fn take_character_data(&mut self) -> CharacterData {
        CharacterData {
            data: self.chr_rom_data.take().unwrap(),
            mirror: self.mirror_mode.clone(),
        }
    }

    pub fn printStats(&self) {
        println!("Mapper: {}", self.mapper);
        println!("Character Mirroring: {:?}", self.mirror_mode);
        match &self.prg_rom_data {
            Some(d) => println!("Program ROM size: {} bytes", d.len()),
            None => println!("Unable to read program rom data. Already taken"),
        };

        match &self.chr_rom_data {
            Some(d) => println!("Character ROM size: {} bytes", d.len()),
            None => println!("Unable to read character rom data. Already taken"),
        };
    }
}

pub struct ProgramData {
    data: Vec<u8>,
}

impl memory::AddressSpace for ProgramData {
    fn peek(&mut self, ptr: u16) -> u8 {
        match self.data.len() {
            l if l < 17000 => match ptr {
                0x8000..=0xBFFF => self.data[((ptr - 0x8000) as usize)],
                0xC000..=0xFFFF => self.data[((ptr - 0xC000) as usize)],
                _ => 0x00,
            },
            _ => match ptr {
                0x8000..=0xBFFF => self.data[((ptr - 0x8000) as usize)],
                0xC000..=0xFFFF => self.data[((ptr - 0x8000) as usize)],
                _ => 0x00,
            },
        }
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        match self.data.len() {
            l if l < 17000 => {
                match ptr {
                    0x8000..=0xBFFF => self.data[((ptr - 0x8000) as usize)] = byte,
                    0xC000..=0xFFFF => self.data[((ptr - 0xC000) as usize)] = byte,
                    _ => (),
                };
            }
            _ => {
                match ptr {
                    0x8000..=0xBFFF => self.data[((ptr - 0x8000) as usize)] = byte,
                    0xC000..=0xFFFF => self.data[((ptr - 0x8000) as usize)] = byte,
                    _ => (),
                };
            }
        }
    }
}

pub struct CharacterData {
    data: Vec<u8>,
    pub mirror: MirrorMode,
}

impl memory::AddressSpace for CharacterData {
    fn peek(&mut self, ptr: u16) -> u8 {
        self.data[ptr as usize]
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        self.data[ptr as usize] = byte
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
