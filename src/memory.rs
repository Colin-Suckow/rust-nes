
use crate::cartridge;

pub trait AddressSpace {
  fn peek(&self, ptr: u16) -> Option<u8>;
  fn poke(&mut self, ptr: u16, byte: u8);
  fn size(&self) -> usize;
}

pub struct Ram {
  data: Vec<u8>,
}

impl Ram {
  pub fn new(size: usize) -> Ram {
    Ram {
      data: vec![0; size],
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
  fn size(&self) -> usize {
    self.data.len()
  }
}

pub struct MappableMem {
  data: Box<dyn AddressSpace>,
  offset: u16,
}

impl MappableMem {
  pub fn new(mem: Box<dyn AddressSpace>, mem_offset: u16) -> MappableMem {
    MappableMem {
      data: mem,
      offset: mem_offset,
    }
  }

  pub fn map_ptr(&self, ptr: u16) -> Option<u16> {
    let data = &*self.data;
    if (ptr <= self.offset + data.size() as u16 && ptr >= self.offset)
      && (ptr - self.offset) < data.size() as u16
    {
      return Some(ptr - self.offset);
    }
    return None;
  }
}

pub struct MemMap {
  mem: Vec<MappableMem>,
}

impl MemMap {
  const MAX_ADDRESS: i32 = 65536;

  pub fn new() -> MemMap {
    MemMap { mem: Vec::new() }
  }
  pub fn add_mappable_mem(&mut self, mem: MappableMem) {
    self.mem.push(mem);
  }

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

impl AddressSpace for MemMap {
  fn peek(&self, ptr: u16) -> Option<u8> {
    for mem in &self.mem {
      match mem.map_ptr(ptr) {
        Some(offset_ptr) => return mem.data.peek(offset_ptr),
        None => {}
      }
    }
    return None;
  }

  fn poke(&mut self, ptr: u16, byte: u8) {
    for mem in &mut self.mem {
      match mem.map_ptr(ptr) {
        Some(offset_ptr) => {
          mem.data.poke(offset_ptr, byte);
        }
        None => {}
      }
    }
  }

  fn size(&self) -> usize {
    let mut total_size = 0;
    for mem in &self.mem {
      total_size += mem.data.size();
    }
    return total_size;
  }
}
