use crate::AddressSpace;

pub struct DummyPPU;

//Returns 128 on all reads to simulate vblank
impl AddressSpace for DummyPPU {
    fn peek(&self, ptr: u16) -> u8 {
        
        128
    }

    fn poke(&mut self, ptr: u16, byte: u8) {
        //println!("PPU WRITE!");
    }
}

