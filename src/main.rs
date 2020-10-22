#![feature(const_if_match)]


extern crate clap;
use clap::{App, Arg};
use std::time::{SystemTime, UNIX_EPOCH};

mod cartridge;
mod cpu;
mod instruction;
mod memory;
mod ppu;

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
        ppu: crate::ppu::DummyPPU,
    };

    bus.write_mem();

    let mut cpu = Cpu::new(bus);

    // for i in (0..255) {
    //     let op = crate::instruction::OPCODES[i].clone();
    //     print!("Address: {:#X} OP: ", i);
    //     match op {
    //         Some(o) => println!("{:?}", o.instruction),
    //         None => println!("None"),
    //     };
    // }

    cpu.reset();

    let mut start = SystemTime::now();

    loop {
        //start = SystemTime::now();
        cpu.step_cycle();
        //let end_time = SystemTime::now();
        //println!("{:?}mhz", ((1.0 / end_time.duration_since(start).unwrap().as_secs_f64()) / 1000000.0).round());
    }
    
}
