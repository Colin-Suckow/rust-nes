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

pub struct Mappable_Mem {
  data: Box<dyn AddressSpace>,
  offset: u16,
}

impl Mappable_Mem {
  pub fn map_ptr(&self, ptr: u16) -> Option<u16> {
    let data = &*self.data;
    if ptr <= self.offset + data.size() as u16 && ptr >= self.offset {
      return Some(ptr - self.offset);
    }
    return None;
  }
}

pub struct Mem_Map {
  mem: Vec<Mappable_Mem>,
}

impl Mem_Map {
  pub fn new() -> Mem_Map {
    Mem_Map { mem: Vec::new() }
  }
  pub fn add_mappable_mem(&mut self, mem: Mappable_Mem) {
    self.mem.push(mem);
  }
}

impl AddressSpace for Mem_Map {
  fn peek(&self, ptr: u16) -> Option<u8> {
    for mem in &self.mem {
      match mem.map_ptr(ptr) {
        Some(offsetPtr) => return mem.data.peek(ptr),
        None => {}
      }
    }
    return None;
  }

  fn poke(&mut self, ptr: u16, byte: u8) {
    for mem in &self.mem {
      match mem.map_ptr(ptr) {
        Some(offsetPtr) => {
          mem.data.poke(offsetPtr, byte);
        },
        None => {}
      }
    }
  }

  fn size(&self) -> usize {
    let totalSize = 0;
    for mem in &self.mem {
      totalSize += mem.data.size();
    }
    return totalSize;
  }
}
