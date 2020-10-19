use crate::cartridge;

pub trait AddressSpace {
    fn peek(&self, ptr: u16) -> Option<u8>;
    fn poke(&mut self, ptr: u16, byte: u8);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_testbus() {
        let bus = TestBus;
        assert_eq!(bus.peek(256).unwrap(), 1 as u8);
    }
}