use crate::cartridge;

pub trait AddressSpace {
    fn peek(&self, ptr: u16) -> Option<u8>;
    fn poke(&mut self, ptr: u16, byte: u8);
    fn peek_16(&self, ptr: u16) -> u16 {
        let byte1 = self.peek(ptr).unwrap();
        let byte2 = self.peek(ptr).unwrap();
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
}

impl AddressSpace for Ram {
    fn peek(&self, ptr: u16) -> Option<u8> {
        Some(self.data[ptr as usize])
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        self.data[ptr as usize] = byte;
    }
}

pub struct Bus {
    pub ram: Ram,
    pub cartridge: cartridge::Cartridge,
}

impl Bus {
    pub fn debug_print_memory(&self) {
        for address in 0..2300 {
            match self.peek(address as u16) {
                Some(val) => print!("{} : {}, ", address, val),
                None => print!("{} : NULL, ", address),
            }
            if address % 5 == 0 {
                print!("\n");
            }
        }
    }
}

impl AddressSpace for Bus {
    fn peek(&self, ptr: u16) -> Option<u8> {
        return match ptr {
            0x0000..=0x07FF => self.ram.peek(ptr),
            0x4020..=0xFFFF => self.cartridge.peek(ptr),
            _ => None,
        };
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        match ptr {
            0x0000..=0x07FF => self.ram.poke(ptr, byte),
            0x4020..=0xFFFF => self.cartridge.poke(ptr, byte),
            _ => (),
        }
    }
}

//Returns the value given as address % 255
pub struct TestBus;

impl AddressSpace for TestBus {
    fn peek(&self, ptr: u16) -> Option<u8> {
        Some((ptr % 255) as u8)
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        ()
    }
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

pub fn absolute_indexed_address(byte1: u8, byte2: u8, register: u8) -> u16 {
    absolute_address(byte1, byte2) + register as u16
}

pub fn zero_page_indexed_address(byte: u8, register: u8) -> u16 {
    byte.wrapping_add(register) as u16
}

pub fn indexed_indirect_address_location(byte: u8, x: u8) -> u16 {
    byte.wrapping_add(x) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_testbus() {
        let bus = TestBus;
        assert_eq!(bus.peek(256).unwrap(), 1 as u8);
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

    #[test]
    fn test_absolute_indexed_address() {
        assert_eq!(absolute_indexed_address(0x10, 0x20, 0x1), 0x2011);
    }

    #[test]
    fn test_zero_page_indexed_address() {
        assert_eq!(zero_page_indexed_address(0xC0, 0x60), 0x0020);
    }

    #[test]
    fn test_indexed_indirect_address_location() {
        assert_eq!(indexed_indirect_address_location(0x20, 0x04), 0x0024);
    }
}