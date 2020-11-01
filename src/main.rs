#![feature(const_if_match)]


extern crate clap;
use clap::{App, Arg};
use std::time::{SystemTime, UNIX_EPOCH};

//use minifb::{Key, Window, WindowOptions, Menu, MenuItem, KeyRepeat};

use pixels::{Pixels, SurfaceTexture};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent, KeyboardInput};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod cartridge;
mod cpu;
mod instruction;
mod memory;
mod ppu;
mod controller;

use cpu::Cpu;
use memory::AddressSpace;
use crate::ppu::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

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

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((DISPLAY_WIDTH * 3) as f64, (DISPLAY_HEIGHT * 3) as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size.clone())
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, surface_texture).unwrap()
    };


    event_loop.run(move |event, _, control_flow| {
        //Handle events
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        match input {
                            KeyboardInput { state, virtual_keycode, ..} => {
                                if let Some(keycode) = virtual_keycode {
                                    cpu.bus.controller.update(&keycode, &state);
                                }
                            }
                        }
                    }
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                pixels.get_frame().copy_from_slice(&cpu.bus.ppu.buffer);
                pixels.render().unwrap();
            },
            _ => ()
        };

        if cpu.bus.ppu.check_nmi() {
            cpu.fire_nmi();
        }

        cpu.step_cycle();

        cpu.bus.ppu.step_cycle();
        cpu.bus.ppu.step_cycle();
        cpu.bus.ppu.step_cycle();

        if cpu.bus.ppu.show_frame() {
            window.request_redraw();
        }


    });




    
}
