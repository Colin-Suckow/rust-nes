use crate::cartridge::{CharacterData, MirrorMode};
use crate::AddressSpace;
use bit_field::BitField;

pub const DISPLAY_WIDTH: usize = 256;
pub const DISPLAY_HEIGHT: usize = 240;

pub struct PPU {
    pub buffer: Vec<u32>,
    character_data: CharacterData,
    vram: Vec<u8>,
    x: u16,
    y: u16,
    PPUCTRL: u8,
    PPUMASK: u8,
    PPUSTATUS: u8,
    OAMADDR: u8,
    OAMDATA: u8,
    PPUSCROLL: u8,
    PPUADDR: u8,
    PPUDATA: u8,
    OAMDMA: u8,
    addr_latch: bool,
    ppuaddr_address: u16,
    nmi_fired: bool,
    oam_mem: Vec<u8>,
    sprites_to_render: Vec<usize>,
}

impl PPU {
    pub fn new(character_rom: CharacterData) -> Self {
        Self {
            buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            character_data: character_rom,
            vram: vec![0x00; 2048],
            x: 0,
            y: 0,
            PPUCTRL: 0,
            PPUMASK: 0,
            PPUSTATUS: 0,
            OAMADDR: 0,
            OAMDATA: 0,
            PPUSCROLL: 0,
            PPUADDR: 0,
            PPUDATA: 0,
            OAMDMA: 0,
            addr_latch: false,
            ppuaddr_address: 0,
            nmi_fired: false,
            oam_mem: vec![0; 256],
            sprites_to_render: Vec::<usize>::new(),
        }
    }

    pub fn step_cycle(&mut self) {
        self.update_position();

        self.update_status_register();

        if !self.check_vblank() {
            let half = match self.PPUCTRL.get_bit(3) {
                true => TableHalf::Left,
                false => TableHalf::Right,
            };

            let offset = match self.PPUCTRL & 0x3 {
                0 => 0x2000,
                1 => 0x2400,
                2 => 0x2800,
                3 => 0x2C00,
                _ => 0,
            };

            let col = (self.x / 8) as u16;
            let row = (self.y / 8) as u16;
            let addr = ((row * 32) + col) as u16 + offset;

            let tile_val = self.peek_vram(addr);
            let tcol = tile_val & 0xF;
            let trow = tile_val >> 4;

            let val = self.get_background_pixel_value(
                half,
                tcol as i32,
                trow as i32,
                (self.x % 8) as i32,
                (self.y % 8) as i32,
            );
            //let color = if val > 0 { 0xFFFFFFFF } else { 0 };
            let color: u32 = match val {
                1 => 0xFF0000FF,
                2 => 0x00FF00FF,
                3 => 0x0000FFFF,
                _ => 0x0,
            };
            let mx = self.x.clone() as usize;
            let my = self.y.clone() as usize;
            self.set_pixel(mx, my, color);
        }
    }

    // fn update_sprites_to_render(&mut self) {
    //     self.sprites_to_render.clear();
    //     for (i, sprite) in self.oam_mem.chunks(4).enumerate() {
    //         if sprite[0] >= y && sprite[0] < y+8 {
    //             self.sprites_to_render.push(i)
    //         }
    //     }
    // }

    pub fn check_nmi(&mut self) -> bool {
        if self.PPUCTRL.get_bit(7) && self.y == 240 && !self.nmi_fired {
            self.nmi_fired = true;
            true
        } else {
            false
        }
    }

    fn update_status_register(&mut self) {
        self.PPUSTATUS.set_bit(7, self.check_vblank());
    }

    pub fn show_frame(&mut self) -> bool {
        if self.y == 0 && self.nmi_fired {
            self.nmi_fired = false;

            let half = match self.PPUCTRL.get_bit(3) {
                true => TableHalf::Right,
                false => TableHalf::Left,
            };

            for sprite in self.oam_mem.clone().chunks(4) {
                let tcol = sprite[1] & 0xF;
                let trow = sprite[1] >> 4;
                self.draw_tile(
                    half,
                    sprite[3].clone() as i32,
                    sprite[0].clone() as i32,
                    tcol.clone() as i32,
                    trow.clone() as i32,
                    sprite[2].get_bit(6),
                    sprite[2].get_bit(7),
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
                _ => (),
            },
            MirrorMode::Horizontal => match ptr {
                0x2000..=0x23FF => self.vram[(ptr - 0x2000) as usize] = byte,
                0x2400..=0x27FF => self.vram[(ptr - 0x2000) as usize] = byte,
                0x2800..=0x2BFF => self.vram[(ptr - 0x2400) as usize] = byte,
                0x2C00..=0x2FFF => self.vram[(ptr - 0x2800) as usize] = byte,
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
                _ => 0,
            },
            MirrorMode::Horizontal => match ptr {
                0x2000..=0x23FF => self.vram[(ptr - 0x2000) as usize],
                0x2400..=0x27FF => self.vram[(ptr - 0x2000) as usize],
                0x2800..=0x2BFF => self.vram[(ptr - 0x2400) as usize],
                0x2C00..=0x2FFF => self.vram[(ptr - 0x2800) as usize],
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
                let val =
                    self.get_background_pixel_value(half.clone(), tile_column, tile_row, mc, mr);
                let color: u32 = match val {
                    1 => 0xFF0000FF,
                    2 => 0x00FF00FF,
                    3 => 0x0000FFFF,
                    _ => 0x0,
                };
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
            0x2000 => self.PPUCTRL,
            0x2001 => self.PPUMASK,
            0x2002 => {
                self.addr_latch = false;
                self.PPUSTATUS
            }
            0x2003 => self.OAMADDR,
            0x2004 => self.oam_mem[self.OAMADDR as usize],
            0x2005 => self.PPUSCROLL,
            0x2006 => self.PPUADDR,
            0x2007 => self.PPUDATA,
            _ => 0,
        }
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        match ptr {
            0x2000 => self.PPUCTRL = byte,
            0x2001 => self.PPUMASK = byte,
            0x2002 => self.PPUSTATUS = byte,
            0x2003 => self.OAMADDR = byte,
            0x2004 => {
                //OAMDATA
                self.oam_mem[self.OAMADDR as usize] = byte;
                self.OAMADDR += 1;
            }
            0x2005 => self.PPUSCROLL = byte,
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
                self.ppuaddr_address += if self.PPUCTRL.get_bit(2) { 32 } else { 1 };
            }
            _ => (),
        }
    }
}

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

    address = address | y_offset as u16; //y offset
    address = address | bit_plane_mask; //Upper or lower bit plane
    address = address | (tile_column as u16) << 4; //Tile column
    address = address | (tile_row as u16) << 8;
    address = address | table_half_mask;
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
