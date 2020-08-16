use crate::instruction::Operation;
use crate::memory::Bus;
//http://nesdev.com/6502_cpu.txt
pub struct Cpu {
    bus: Bus,
    //Registers
    PC: u16, //Program counter
    S: u8,   //Stack pointer
    P: u8,   //Processor status
    A: u8,   //Accumulator
    X: u8,   //Index X
    Y: u8,   //Index Y
    current_instruction: Option<Operation>,
    instruction_progress: u8,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            bus: bus,
            PC: 0,
            S: 0,
            P: 0,
            A: 0,
            X: 0,
            Y: 0,
            current_instruction: None,
            instruction_progress: 0,
        }
    }

    pub fn step_cycle(&mut self) {
        todo!();
    }
}