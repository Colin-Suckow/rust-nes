mod cartridge;
mod controller;
mod cpu;
mod instruction;
mod memory;
mod ppu;

use controller::ControllerState;
use cpu::Cpu;
use cartridge::{MirrorMode, Cartridge};

pub mod prelude {
    pub use super::controller::ControllerState;
    pub use super::Emulator;
}
pub struct Emulator {
    cpu: Cpu<memory::Bus>,
    framebuffer: Vec<u32>,
}

impl Emulator {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let mut rom = cartridge::Cartridge::load(rom_data);
        println!("Mirror mode: {:?}", rom.mirror_mode);

        let ppu = crate::ppu::PPU::new(rom.take_character_data());

        let controller = controller::Controller::new();

        let mut bus = memory::Bus {
            ram: memory::Ram::new(),
            cartridge: rom.take_program_data(),
            ppu,
            controller,
        };


        let mut cpu = Cpu::new(bus);

        cpu.reset();

        Self {
            cpu,
            framebuffer: vec![0; 256 * 240],
        }
    }

    fn step_cycle(&mut self) {
        if self.cpu.bus.ppu.check_nmi() {
            self.cpu.fire_nmi();
        }

        self.cpu.step_cycle();

        self.cpu.bus.ppu.step_cycle();
        self.cpu.bus.ppu.step_cycle();
        self.cpu.bus.ppu.step_cycle();
    }

    pub fn run_frame(&mut self) {
        loop {
            self.step_cycle();
            if self.cpu.bus.ppu.show_frame() {
                //Render frame
                self.framebuffer = self.cpu.bus.ppu.buffer.clone();

                return;
            }
        }
    }

    pub fn update_controller_state(&mut self, state: ControllerState) {
        self.cpu.bus.controller.update_controller(state);
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.framebuffer
    }

    pub fn nametable_buffer(&mut self) -> Vec<u32> {
        self.cpu.bus.ppu.render_nametable().clone()
    }
}
