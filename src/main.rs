extern crate clap;
use clap::{Arg, App, SubCommand};

mod cartridge;
mod memory;

use memory::AddressSpace;

fn main() {

    let matches = App::new("rust-nes")
                    .version("1.0")
                    .author("Colin Suckow")
                    .about("A Nintendo emulator written in rust. My first rust project")
                    .arg(Arg::with_name("exec")
                            .short("e")
                            .long("execute")
                            .value_name("FILE")
                            .help("ROM file to load")
                            .takes_value(true))
                    .get_matches();

    let rom_path = match matches.value_of("exec") {
        Some(val) => val,
        None => {
            println!("ERROR: No file provided");
            return;
        }
    };

    println!("Recived argument {}", rom_path);

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
    cartridge::Cartridge::load(rom_path);
}
