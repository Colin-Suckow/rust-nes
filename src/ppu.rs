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
}

impl PPU {
    pub fn new(character_rom: CharacterData) -> Self {
        Self {
            primary_buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            secondary_buffer: vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            primary_buffer_active: true,
            character_data: character_rom,
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
        for r in 0..16 {
            for c in 0..16 {
                self.draw_tile(c * 8, r * 8, r, c);
            }
        }
        
        self.swap_buffer();
    }

    fn draw_tile(&mut self, x: i32, y: i32, tile_column: i32, tile_row: i32) {
        for r in 0..8 {
            for c in 0..8 {
                let val = self.get_pixel_value(TableHalf::Right, tile_column, tile_row, c, r);
                let color = if val > 0 { 0xFFFFFFFF } else { 0 };
                self.get_inactive_buffer_mut()[(((r + y) * DISPLAY_WIDTH as i32) + (c + x)) as usize] = color;
            }
        }
    }

    fn get_pixel_value(
        &self,
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

//Returns 128 on all reads to simulate vblank
impl AddressSpace for PPU {
    fn peek(&self, ptr: u16) -> u8 {
        128
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        //println!("PPU WRITE!");
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
        let addr = map_tile_address(TableHalf::Left, 4, 14, 3, BitPlane::Lower);
        assert_eq!(addr, 0b001110_01000011);
    }
}
