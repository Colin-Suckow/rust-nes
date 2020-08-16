use crate::instruction::{Operation, Instruction, AddressingMode};
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
    current_operation: Option<Operation>,
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
            current_operation: None,
            operation_progress: 0,
        }
    }

    pub fn step_cycle(&mut self) {

        if self.current_operation.is_some() && self.operation_progress < self.current_operation.unwrap().base_cycle_count {
            self.operation_progress += 1;
            return;
        } else {
            self.current_operation = self.consume_next_operation();
            self.operation_progress = 0;
        }

        let operation = self.current_operation.unwrap();

        
    }

    pub fn consume_next_operation(&mut self) -> Option<Operation> {
        todo!();
    }
}