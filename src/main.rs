#![feature(const_if_match)]

extern crate clap;
use clap::{App, Arg};
use std::time::{SystemTime, UNIX_EPOCH};

//use minifb::{Key, Window, WindowOptions, Menu, MenuItem, KeyRepeat};

use minifb::{Key, Window, WindowOptions};

mod cartridge;
mod controller;
mod cpu;
mod instruction;
mod memory;
mod ppu;

use crate::ppu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
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

    let mut rom = cartridge::Cartridge::load(rom_path);

    rom.printStats();

    let mut ppu = crate::ppu::PPU::new(rom.take_character_data());

    let controller = controller::Controller::new();

    let mut bus = memory::Bus {
        ram: memory::Ram::new(),
        cartridge: rom.take_program_data(),
        ppu: ppu,
        controller: controller,
    };

    bus.write_mem();

    let mut cpu = Cpu::new(bus);

    cpu.reset();

    // let mut window = Window::new(
    //     "NES emulator",
    //     crate::ppu::DISPLAY_WIDTH * 3,
    //     crate::ppu::DISPLAY_HEIGHT * 3,
    //     WindowOptions::default()
    // ).unwrap();

    let mut window = Window::new(
        "Test - ESC to exit",
        DISPLAY_WIDTH * 3,
        DISPLAY_HEIGHT * 3,
        WindowOptions::default(),
    ).unwrap();

    'game_loop: loop {
        if cpu.bus.ppu.check_nmi() {
            cpu.fire_nmi();
        }

        cpu.step_cycle();

        cpu.bus.ppu.step_cycle();
        cpu.bus.ppu.step_cycle();
        cpu.bus.ppu.step_cycle();
        

        if cpu.bus.ppu.show_frame() {
            //Render frame
            window.update_with_buffer(&cpu.bus.ppu.buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();

            //Handle events
            if !window.is_open() {
                break 'game_loop;
            }
        }
    }
}
