use crate::cartridge::CharacterData;
use crate::AddressSpace;
use bit_field::BitField;

pub const DISPLAY_WIDTH: usize = 256;
pub const DISPLAY_HEIGHT: usize = 240;

pub struct PPU {
    primary_buffer: Vec<u32>,
    secondary_buffer: Vec<u32>,
    primary_buffer_active: bool,
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
}

impl PPU {
    pub fn new(character_rom: CharacterData) -> Self {
        Self {
            primary_buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            secondary_buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            primary_buffer_active: true,
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
            ppuaddr_address: 0
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        match self.primary_buffer_active {
            true => &self.primary_buffer,
            false => &self.secondary_buffer,
        }
    }

    pub fn get_inactive_buffer_mut(&mut self) -> &mut Vec<u32> {
        match self.primary_buffer_active {
            true => &mut self.secondary_buffer,
            false => &mut self.primary_buffer,
        }
    }

    fn swap_buffer(&mut self) {
        self.primary_buffer_active = !self.primary_buffer_active;
    }

    pub fn step_cycle(&mut self) {
        self.update_position();

        self.update_status_register();

        if !self.check_vblank() {
            let half = match self.PPUCTRL.get_bit(3) {
                true => TableHalf::Left,
                false => TableHalf::Right,
            };
            let col = (self.x.clone() / 32) as u8;
            let row = (self.y.clone() / 30) as u8;
            let addr = ((col | (row << 4)) as u16) + 0x2000;

            let tile_val = self.peek_vram(addr);
            let tcol = tile_val & 0xF;
            let trow = tile_val >> 4;

            let val = self.get_pixel_value(half, tcol as i32, trow as i32, (self.x % 7) as i32, (self.y % 7) as i32);
            let color = if val > 0 { 0xFFFFFFFF } else { 0 };
            let mx = self.x.clone();
            let my = self.y.clone();
            self.get_inactive_buffer_mut()[((my * DISPLAY_WIDTH as u16) + mx) as usize] = color;
        }

        

        if self.y == 240 && self.x == 0 {
            self.swap_buffer(); 
        }
    }

    pub fn check_nmi(&self) -> bool {
        self.PPUCTRL.get_bit(7) && self.x == 0 && self.y == 240
    }

    fn update_status_register(&mut self) {
        self.PPUSTATUS.set_bit(7, self.check_vblank());
    }

    pub fn show_frame(&self) -> bool {
        self.y == 0 && self.x == 0
    }

    fn peek_nametable(&mut self, ptr: u16) -> u8 {
        self.character_data.peek(ptr)
        //Handle mirroring later
        // let mirror_ptr = match self.character_data.mirror {
        //     crate::cartridge::MirrorMode::Vertical => {
        //         match ptr {
        //             0x2000..=
        //         }
        //     },
        //     crate::cartridge::MirrorMode::Horizontal => {}
        // }
    }

    fn poke_nametable(&mut self, ptr: u16, byte: u8) {
        self.character_data.poke(ptr - 0x2000, byte);
    }

    fn poke_vram(&mut self, ptr:u16, byte: u8) {
        self.vram[ptr as usize] = byte;
    }

    fn peek_vram(&self, ptr: u16) -> u8 {
        self.vram[ptr as usize - 0x2000]
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

        if self.y >= 262 {
            self.y = 0;
        };

        //println!("X: {} Y:{}", self.x, self.y);
    }

    fn draw_tile(&mut self, x: i32, y: i32, tile_column: i32, tile_row: i32) {
        for r in 0..8 {
            for c in 0..8 {
                let val = self.get_pixel_value(TableHalf::Left, tile_column, tile_row, c, r);
                let color = if val > 0 { 0xFFFFFFFF } else { 0 };
                self.get_inactive_buffer_mut()
                    [(((r + y) * DISPLAY_WIDTH as i32) + (c + x)) as usize] = color;
            }
        }
    }

    fn get_pixel_value(
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
            },
            0x2003 => self.OAMADDR,
            0x2004 => self.OAMDATA,
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
            0x2004 => self.OAMDATA = byte,
            0x2005 => self.PPUSCROLL = byte,
            0x2006 => {
                //PPUADDR
                self.ppuaddr_address = match self.addr_latch {
                    false => byte as u16,
                    true => self.ppuaddr_address | ((byte as u16) << 8)
                };
            },
            0x2007 => {
                //PPUDATA
                self.poke_vram(self.ppuaddr_address, byte);
                self.ppuaddr_address += if self.PPUCTRL.get_bit(2) {32} else {1};
            },
            _ => (),
        }
    }
}

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
