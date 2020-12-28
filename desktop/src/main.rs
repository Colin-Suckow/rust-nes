extern crate clap;
use clap::{App, Arg};
use minifb::{Key, Window, WindowOptions};
use nesemu::prelude::*;
use image::{RgbImage, Rgb, ImageBuffer};

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
            keys.contains(&Key::Apostrophe),
            keys.contains(&Key::Semicolon),
            keys.contains(&Key::Enter),
            keys.contains(&Key::RightShift),
        ));

        //Save image of nametable
        if window.is_key_released(Key::N) {
            println!("Saving nametable...");
            let buffer: Vec<[u8;3]> = emu.nametable_buffer().iter().map(|x| {
                [((x >> 16) & 255) as u8, ((x >> 8) & 255) as u8, ((x >> 0) & 255) as u8]
            }).collect();
            let mut img = RgbImage::new(512, 480);
            for (index, pixel) in buffer.iter().enumerate() {
                img.put_pixel((index as u32) % 512, (index as u32) / 512, Rgb(pixel.to_owned()));
            }
            img.save("nametable.png");
        }
    }
}
