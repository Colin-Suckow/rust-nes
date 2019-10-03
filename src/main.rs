extern crate clap;
use clap::{App, Arg};

mod cartridge;
mod memory;

fn main() {
    let matches = App::new("rust-nes")
        .version("1.0")
        .author("Colin Suckow")
        .about("A Nintendo emulator written in rust. My first rust project")
        .arg(
            Arg::with_name("exec")
                .short("e")
                .long("execute")
                .value_name("FILE")
                .help("ROM file to load")
                .takes_value(true),
        )
        .get_matches();

    let rom_path = match matches.value_of("exec") {
        Some(val) => val,
        None => {
            println!("ERROR: No file provided");
            return;
        }
    };

    println!("Recived argument {}", rom_path);

    let mut memory_map = memory::MemMap::new();

    //RAM
    memory_map.add_mappable_mem(memory::MappableMem::new(
        Box::new(memory::Ram::new(2048)),
        0,
    ));

    let rom = cartridge::Cartridge::load(rom_path);

    rom.printStats();

    //Cartridge
    memory_map.add_mappable_mem(memory::MappableMem::new(Box::new(rom), 0x4020));
}
