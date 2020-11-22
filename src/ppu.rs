use crate::cartridge::{CharacterData, MirrorMode};
use crate::memory::AddressSpace;
use bit_field::BitField;

pub const DISPLAY_WIDTH: usize = 256;
pub const DISPLAY_HEIGHT: usize = 240;

enum PaletteRam {
    Background0,
    Background1,
    Background2,
    Background3,
    Sprite0,
    Sprite1,
    Sprite2,
    Sprite3,
}

pub struct PPU {
    pub buffer: Vec<u32>,
    character_data: CharacterData,
    vram: Vec<u8>,
    x: u16,
    y: u16,
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    addr_latch: bool,
    ppuaddr_address: u16,
    nmi_fired: bool,
    oam_mem: Vec<u8>,
    palette_ram: Vec<u8>,
    scroll_x: u8,
    scroll_y: u8,
    scroll_latch: bool,
}

impl PPU {
    pub fn new(character_rom: CharacterData) -> Self {
        Self {
            buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            character_data: character_rom,
            vram: vec![0x00; 2048],
            x: 0,
            y: 0,
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            addr_latch: false,
            ppuaddr_address: 0,
            nmi_fired: false,
            oam_mem: vec![0; 256],
            palette_ram: vec![0; 0x0020],
            scroll_x: 0,
            scroll_y: 0,
            scroll_latch: false,
        }
    }

    pub fn step_cycle(&mut self) {
        self.update_position();

        self.update_status_register();

        if !self.check_vblank() {
            let half = match self.ppuctrl.get_bit(3) {
                true => TableHalf::Left,
                false => TableHalf::Right,
            };

            let nametable = self.ppuctrl & 0x3;
            let offset = match nametable {
                0 => 0x2000,
                1 => 0x2400,
                2 => 0x2800,
                3 => 0x2C00,
                _ => 0,
            };

            let col = (self.x.wrapping_add(self.scroll_x as u16) / 8) as u16;
            let row = (self.y.wrapping_add(self.scroll_y as u16) / 8) as u16;
            let addr = ((row * 32) + col) as u16 + offset;

            let tile_val = self.peek_vram(addr);
            let tcol = tile_val & 0xF;
            let trow = tile_val >> 4;

            let val = self.get_background_pixel_value(
                half,
                tcol as i32,
                trow as i32,
                (self.x.wrapping_add(self.scroll_x as u16) % 8) as i32,
                (self.y.wrapping_add(self.scroll_y as u16) % 8) as i32,
            );
            //let color = if val > 0 { 0xFFFFFFFF } else { 0 };
            let palette_segment = self.get_background_palette_segment(
                nametable as usize,
                self.x.wrapping_add(self.scroll_x as u16) as usize,
                self.y.wrapping_add(self.scroll_y as u16) as usize,
            );

            let color = self.get_palette_color(&palette_segment, val as u16);
            let mx = self.x as usize;
            let my = self.y as usize;
            self.set_pixel(mx, my, color);
        }
    }

    fn get_background_palette_segment(&self, nametable: usize, x: usize, y: usize) -> PaletteRam {
        let offset = match nametable {
            0 => 0x23C0,
            1 => 0x27C0,
            2 => 0x2BC0,
            3 => 0x2FC0,
            _ => 0,
        };
        let px = x / 32;
        let py = y / 32;
        let index = (py * 8) + px;
        let pbyte = self.peek_vram((index + offset) as u16);
        let right = ((px * 32) + x) % 32 >= 16;
        let bottom = ((py * 32) + y) % 32 >= 16;
        let val = match (bottom, right) {
            (false, false) => pbyte & 0x3,       //topleft
            (false, true) => (pbyte >> 2) & 0x3, //topright
            (true, false) => (pbyte >> 4) & 0x3, //bottomleft
            (true, true) => (pbyte >> 6) & 0x3,  //bottomright
        };

        match val {
            1 => PaletteRam::Background1,
            2 => PaletteRam::Background2,
            3 => PaletteRam::Background3,
            _ => PaletteRam::Background0,
        }
    }

    fn get_palette_color(&self, location: &PaletteRam, val: u16) -> u32 {
        let color_code = {
            if val == 0 {
                self.peek_vram(0x3F00) //Universal background
            } else {
                match location {
                    PaletteRam::Background0 => self.peek_vram((0x3F00 + val) as u16),
                    PaletteRam::Background1 => self.peek_vram((0x3F04 + val) as u16),
                    PaletteRam::Background2 => self.peek_vram((0x3F08 + val) as u16),
                    PaletteRam::Background3 => self.peek_vram((0x3F0C + val) as u16),
                    PaletteRam::Sprite0 => self.peek_vram((0x3F10 + val) as u16),
                    PaletteRam::Sprite1 => self.peek_vram((0x3F14 + val) as u16),
                    PaletteRam::Sprite2 => self.peek_vram((0x3F18 + val) as u16),
                    PaletteRam::Sprite3 => self.peek_vram((0x3F1C + val) as u16),
                }
            }
        };
        PALETTE[color_code as usize]
    }

    pub fn check_nmi(&mut self) -> bool {
        if self.ppuctrl.get_bit(7) && self.y == 240 && !self.nmi_fired {
            self.nmi_fired = true;
            true
        } else {
            false
        }
    }

    fn update_status_register(&mut self) {
        self.ppustatus.set_bit(7, self.check_vblank());
    }

    pub fn show_frame(&mut self) -> bool {
        if self.y == 0 && self.nmi_fired {
            self.nmi_fired = false;

            let half = match self.ppuctrl.get_bit(3) {
                true => TableHalf::Right,
                false => TableHalf::Left,
            };

            for sprite in self.oam_mem.clone().chunks(4) {
                let tcol = sprite[1] & 0xF;
                let trow = sprite[1] >> 4;
                let segment = match sprite[2] & 0x3 {
                    1 => PaletteRam::Sprite1,
                    2 => PaletteRam::Sprite2,
                    3 => PaletteRam::Sprite3,
                    _ => PaletteRam::Sprite0,
                };
                self.draw_tile(
                    half,
                    sprite[3] as i32,
                    sprite[0] as i32,
                    tcol as i32,
                    trow as i32,
                    sprite[2].get_bit(6),
                    sprite[2].get_bit(7),
                    &segment,
                );
            }

            true
        } else {
            false
        }
    }

    fn poke_vram(&mut self, ptr: u16, byte: u8) {
        match self.character_data.mirror {
            MirrorMode::Vertical => match ptr {
                0x2000..=0x23FF => self.vram[(ptr - 0x2000) as usize] = byte,
                0x2400..=0x27FF => self.vram[(ptr - 0x2000) as usize] = byte,
                0x2800..=0x2BFF => self.vram[(ptr - 0x2800) as usize] = byte,
                0x2C00..=0x2FFF => self.vram[(ptr - 0x2400) as usize] = byte,
                0x3F00..=0x3F1F => self.palette_ram[(ptr - 0x3F00) as usize] = byte,
                _ => (),
            },
            MirrorMode::Horizontal => match ptr {
                0x2000..=0x23FF => self.vram[(ptr - 0x2000) as usize] = byte,
                0x2400..=0x27FF => self.vram[(ptr - 0x2000) as usize] = byte,
                0x2800..=0x2BFF => self.vram[(ptr - 0x2400) as usize] = byte,
                0x2C00..=0x2FFF => self.vram[(ptr - 0x2800) as usize] = byte,
                0x3F00..=0x3F1F => self.palette_ram[(ptr - 0x3F00) as usize] = byte,
                _ => (),
            },
        }
    }

    fn peek_vram(&self, ptr: u16) -> u8 {
        match self.character_data.mirror {
            MirrorMode::Vertical => match ptr {
                0x2000..=0x23FF => self.vram[(ptr - 0x2000) as usize],
                0x2400..=0x27FF => self.vram[(ptr - 0x2000) as usize],
                0x2800..=0x2BFF => self.vram[(ptr - 0x2800) as usize],
                0x2C00..=0x2FFF => self.vram[(ptr - 0x2400) as usize],
                0x3F00..=0x3F1F => self.palette_ram[(ptr - 0x3F00) as usize],
                _ => 0,
            },
            MirrorMode::Horizontal => match ptr {
                0x2000..=0x23FF => self.vram[(ptr - 0x2000) as usize],
                0x2400..=0x27FF => self.vram[(ptr - 0x2000) as usize],
                0x2800..=0x2BFF => self.vram[(ptr - 0x2400) as usize],
                0x2C00..=0x2FFF => self.vram[(ptr - 0x2800) as usize],
                0x3F00..=0x3F1F => self.palette_ram[(ptr - 0x3F00) as usize],
                _ => 0,
            },
        }
    }

    fn check_vblank(&self) -> bool {
        self.y >= 239
    }

    fn update_position(&mut self) {
        if self.x >= DISPLAY_WIDTH as u16 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        };

        if self.y >= 261 {
            self.y = 0;
        };

        //println!("X: {} Y:{}", self.x, self.y);
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = (y * DISPLAY_WIDTH) + x;
        // for i in 0..3 {
        //     self.buffer[index + i] = color[0 + i];
        // }
        self.buffer[index] = color;
    }

    fn draw_tile(
        &mut self,
        half: TableHalf,
        x: i32,
        y: i32,
        tile_column: i32,
        tile_row: i32,
        mirror_h: bool,
        mirror_v: bool,
        palette_segment: &PaletteRam,
    ) {
        for r in 0..8 {
            for c in 0..8 {
                let mut mr = r;
                let mut mc = c;
                if mirror_v {
                    mr = 7 - r;
                }
                if mirror_h {
                    mc = 7 - c;
                }
                let val = self.get_background_pixel_value(half, tile_column, tile_row, mc, mr);
                if val == 0 {
                    continue;
                }

                let color = self.get_palette_color(palette_segment, val as u16);
                if ((((r + y) * DISPLAY_WIDTH as i32) + (c + x)) as usize) < 61440 {
                    self.set_pixel((c + x) as usize, (r + y) as usize, color);
                };
            }
        }
    }

    fn get_background_pixel_value(
        &mut self,
        table_half: TableHalf,
        tile_column: i32,
        tile_row: i32,
        column: i32,
        row: i32,
    ) -> u8 {
        let lower_byte = self.character_data.peek(map_tile_address(
            &table_half,
            tile_column,
            tile_row,
            row,
            BitPlane::Lower,
        ));
        let upper_byte = self.character_data.peek(map_tile_address(
            &table_half,
            tile_column,
            tile_row,
            row,
            BitPlane::Upper,
        ));
        let lower_bit = lower_byte.get_bit(7 - column as usize) as u8;
        let upper_bit = upper_byte.get_bit(7 - column as usize) as u8;
        lower_bit | (upper_bit << 1)
    }

    pub fn write_dma(&mut self, data: &[u8]) {
        //self.oam_mem.clear();
        self.oam_mem.copy_from_slice(data);
    }
}

// fn apply_palette(value: u8) -> u32 {
//     match value {
//         1 =>
//     }
// }

impl AddressSpace for PPU {
    fn peek(&mut self, ptr: u16) -> u8 {
        match ptr {
            0x2000 => self.ppuctrl,
            0x2001 => self.ppumask,
            0x2002 => {
                self.addr_latch = false;
                self.scroll_latch = false;
                self.ppustatus
            }
            0x2003 => self.oamaddr,
            0x2004 => self.oam_mem[self.oamaddr as usize],
            0x2005 => self.ppuscroll,
            0x2006 => self.ppuaddr,
            0x2007 => self.ppudata,
            _ => 0,
        }
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        match ptr {
            0x2000 => self.ppuctrl = byte,
            0x2001 => self.ppumask = byte,
            0x2002 => self.ppustatus = byte,
            0x2003 => self.oamaddr = byte,
            0x2004 => {
                //OAMDATA
                self.oam_mem[self.oamaddr as usize] = byte;
                self.oamaddr += 1;
            }
            0x2005 => {
                if self.scroll_latch {
                    self.scroll_y = byte;
                } else {
                    self.scroll_x = byte;
                    self.scroll_latch = true;
                }
            }
            0x2006 => {
                //PPUADDR
                self.ppuaddr_address = match self.addr_latch {
                    false => {
                        self.addr_latch = true;
                        byte as u16
                    }
                    true => self.ppuaddr_address << 8 | (byte as u16),
                };
                //println!("{:#X} {:#X}", byte, self.ppuaddr_address);
            }
            0x2007 => {
                //PPUDATA
                //println!("{:#X} {:#X}", byte, self.ppuaddr_address);
                self.poke_vram(self.ppuaddr_address, byte);
                self.ppuaddr_address += if self.ppuctrl.get_bit(2) { 32 } else { 1 };
            }
            _ => (),
        }
    }
}

//RGB palette values from emudev.de
const PALETTE: [u32; 64] = [
    0x7C7C7C, 0x0000FC, 0x0000BC, 0x4428BC, 0x940084, 0xA80020, 0xA81000, 0x881400, 0x503000,
    0x007800, 0x006800, 0x005800, 0x004058, 0x000000, 0x000000, 0x000000, 0xBCBCBC, 0x0078F8,
    0x0058F8, 0x6844FC, 0xD800CC, 0xE40058, 0xF83800, 0xE45C10, 0xAC7C00, 0x00B800, 0x00A800,
    0x00A844, 0x008888, 0x000000, 0x000000, 0x000000, 0xF8F8F8, 0x3CBCFC, 0x6888FC, 0x9878F8,
    0xF878F8, 0xF85898, 0xF87858, 0xFCA044, 0xF8B800, 0xB8F818, 0x58D854, 0x58F898, 0x00E8D8,
    0x787878, 0x000000, 0x000000, 0xFCFCFC, 0xA4E4FC, 0xB8B8F8, 0xD8B8F8, 0xF8B8F8, 0xF8A4C0,
    0xF0D0B0, 0xFCE0A8, 0xF8D878, 0xD8F878, 0xB8F8B8, 0xB8F8D8, 0x00FCFC, 0xF8D8F8, 0x000000,
    0x000000,
];

#[derive(Clone, Copy)]
enum TableHalf {
    Left,
    Right,
}

enum BitPlane {
    Upper,
    Lower,
}

fn map_tile_address(
    table_half: &TableHalf,
    tile_column: i32,
    tile_row: i32,
    y_offset: i32,
    bit_plane: BitPlane,
) -> u16 {
    let mut address: u16 = 0;

    let bit_plane_mask: u16 = match bit_plane {
        BitPlane::Upper => 1,
        BitPlane::Lower => 0,
    } << 3;

    let table_half_mask: u16 = match table_half {
        TableHalf::Left => 0,
        TableHalf::Right => 1,
    } << 12;

    address |= y_offset as u16; //y offset
    address |= bit_plane_mask; //Upper or lower bit plane
    address |= (tile_column as u16) << 4; //Tile column
    address |= (tile_row as u16) << 8;
    address |= table_half_mask;
    address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_tile_address_lower() {
        let addr = map_tile_address(&TableHalf::Left, 4, 14, 3, BitPlane::Lower);
        assert_eq!(addr, 0b001110_01000011);
    }
}
