use crate::instruction::{AddressingMode, Instruction, Operation};
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
    operation_progress: u8,
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
            operation_progress: 0,
        }
    }

    pub fn step_cycle(&mut self) {
        //skip cycle if instruction is still in progress
        if self.operation_progress > 0 {
            self.operation_progress -= 1;
            return;
        }
    }

    fn consume_next_operation(&mut self) -> Option<Operation> {
        todo!();
    }
}

fn parse_instruction_bytes(data: &Vec<u8>) -> Operation {
    todo!();
}
