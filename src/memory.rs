use std::io::Write;

use crate::ppu;
use crate::{cartridge, controller};

pub trait AddressSpace {
    fn peek(&mut self, ptr: u16) -> u8;
    fn poke(&mut self, ptr: u16, byte: u8);
    fn peek_16(&mut self, ptr: u16) -> u16 {
        let byte1 = self.peek(ptr);
        let byte2 = self.peek(ptr + 1);
        u16::from_le_bytes([byte1, byte2])
    }
}

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            data: vec![0; 0x0800],
        }
    }

    pub fn get_dma_data(&self, start: u8) -> &[u8] {
        &self.data[(((start as u16) << 8) as usize)..((((start as u16) << 8) + 256) as usize)]
    }
}

impl AddressSpace for Ram {
    fn peek(&mut self, ptr: u16) -> u8 {
        self.data[ptr as usize]
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        self.data[ptr as usize] = byte;
    }
}

pub struct Bus {
    pub ram: Ram,
    pub cartridge: cartridge::ProgramData,
    pub ppu: ppu::PPU,
    pub controller: controller::Controller,
}

impl Bus {
    #[allow(dead_code)]
    pub fn debug_print_memory(&mut self) {
        for address in 0..0xFFFF {
            let val = self.peek(address as u16);
            print!("{:#X} : {}, ", address, val);
            if address % 5 == 0 {
                print!("\n");
            }
        }
    }

    pub fn write_mem(&mut self) {
        let mut file = std::fs::File::create("mem.txt").unwrap();
        for address in 0..0x10000 {
            let val = self.peek(address as u16);
            file.write_all(format!("{:#X} : {:#X}, ", address, val).as_bytes())
                .unwrap();
            if address % 5 == 0 {
                file.write_all("\n".as_bytes()).unwrap();
            }
        }
    }
}

impl AddressSpace for Bus {
    fn peek(&mut self, ptr: u16) -> u8 {
        match ptr {
            0x4016 => self.controller.peek(ptr), //Controller port 1
            0x4017 => 0,                         //Empty controller 2 hack
            0x0000..=0x07FF => self.ram.peek(ptr),
            0x2000..=0x2007 => self.ppu.peek(ptr),
            0x4020..=0xFFFF => self.cartridge.peek(ptr),
            _ => 0,
        }
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        match ptr {
            0x4014 => {
                //OAM DMA
                self.ppu.write_dma(self.ram.get_dma_data(byte));
            }
            0x4016 => self.controller.poke(ptr, byte),
            0x0000..=0x07FF => self.ram.poke(ptr, byte),
            0x2000..=0x2007 => self.ppu.poke(ptr, byte),
            0x4020..=0xFFFF => self.cartridge.poke(ptr, byte),
            _ => (),
        }
    }
}

//Returns the value given as address % 255
pub struct TestBus;

impl AddressSpace for TestBus {
    fn peek(&mut self, ptr: u16) -> u8 {
        (ptr % 255) as u8
    }

    fn poke(&mut self, _ptr: u16, _byte: u8) {}
}

pub fn relative_address(offset: u8, pc: u16) -> u16 {
    (offset as i8 as i32 + pc as i32) as u16
}

pub fn absolute_address(byte1: u8, byte2: u8) -> u16 {
    u16::from_le_bytes([byte1, byte2])
}

pub fn zero_page_address(byte: u8) -> u16 {
    byte as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_testbus() {
        let mut bus = TestBus;
        assert_eq!(bus.peek(256), 1 as u8);
    }

    #[test]
    fn test_testbus16() {
        let mut bus = TestBus;
        assert_eq!(bus.peek_16(0x20), 0x2120);
    }

    #[test]
    fn test_relative_address() {
        assert_eq!(relative_address(10, 1000), 1010);
        assert_eq!(relative_address(246, 1000), 990);
    }

    #[test]
    fn test_absolute_address() {
        assert_eq!(absolute_address(0x10, 0x20), 0x2010);
    }

    #[test]
    fn test_zero_page_address() {
        assert_eq!(zero_page_address(0x10), 0x0010);
    }
}
