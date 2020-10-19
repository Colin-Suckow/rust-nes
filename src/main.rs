#![feature(const_if_match)]


extern crate clap;
use clap::{App, Arg};

mod cartridge;
mod cpu;
mod instruction;
mod memory;

use cpu::Cpu;
use memory::AddressSpace;

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

    let rom = cartridge::Cartridge::load(rom_path);

    rom.printStats();

    let mut bus = memory::Bus {
        ram: memory::Ram::new(),
        cartridge: rom,
    };

    let mut cpu = Cpu::new(bus);

    cpu.step_cycle();
}
