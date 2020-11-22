extern crate clap;
use clap::{App, Arg};
use minifb::{Key, Window, WindowOptions};
use nesemu::prelude::*;

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
    let data = std::fs::read(rom_path).expect("Failed to read file");

    let mut emu = Emulator::new(data);

    let mut window =
        Window::new("NES Emulator", 256 * 3, 240 * 3, WindowOptions::default()).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    'game_loop: loop {
        emu.run_frame();
        //Render frame
        window.update_with_buffer(emu.buffer(), 256, 240).unwrap();

        //Handle window close
        if !window.is_open() {
            break 'game_loop;
        }

        //Update controller state
        let keys = window.get_keys().unwrap();
        emu.update_controller_state(ControllerState::new(
            keys.contains(&Key::W),
            keys.contains(&Key::S),
            keys.contains(&Key::A),
            keys.contains(&Key::D),
            keys.contains(&Key::Semicolon),
            keys.contains(&Key::Apostrophe),
            keys.contains(&Key::Enter),
            keys.contains(&Key::RightShift),
        ));
    }
}
