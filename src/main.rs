mod cartridge;
mod memory;

use memory::AddressSpace;

fn main() {
    println!("Hello, world!");
    let mut memory_map = memory::MemMap::new();

    //RAM
    memory_map.add_mappable_mem(memory::MappableMem::new(
        Box::new(memory::Ram::new(2048)),
        100,
    ));

    println!("Memory map is {} bytes", memory_map.size());
    memory_map.poke(0, 12);
    memory_map.poke(103, 25);
    println!(
        "loc 0 = {:?}, loc 103 = {:?}",
        memory_map.peek(0),
        memory_map.peek(103)
    );
    cartridge::print_ines_header("BOMBMAN.NES");
}
