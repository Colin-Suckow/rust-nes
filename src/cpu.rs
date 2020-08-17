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

        let operation = self.consume_next_operation().unwrap();

        
    }

    fn consume_next_operation(&mut self) -> Option<Operation> {
        todo!();
    }

    //CPU functions

    fn ADC(&mut self, operation: &Operation) {
        todo!();
    }

    fn AND(&mut self, operation: &Operation) {
        todo!();
    }

    fn ASL(&mut self, operation: &Operation) {
        todo!();
    }

    fn BCC(&mut self, operation: &Operation) {
        todo!();
    }

    fn BCS(&mut self, operation: &Operation) {
        todo!();
    }

    fn BEQ(&mut self, operation: &Operation) {
        todo!();
    }

    fn BIT(&mut self, operation: &Operation) {
        todo!();
    }

    fn BMI(&mut self, operation: &Operation) {
        todo!();
    }

    fn BNE(&mut self, operation: &Operation) {
        todo!();
    }

    fn BPL(&mut self, operation: &Operation) {
        todo!();
    }

    fn BRK(&mut self, operation: &Operation) {
        todo!();
    }

    fn BVC(&mut self, operation: &Operation) {
        todo!();
    }

    fn BVS(&mut self, operation: &Operation) {
        todo!();
    }

    fn CLC(&mut self, operation: &Operation) {
        todo!();
    }

    fn CLD(&mut self, operation: &Operation) {
        todo!();
    }

    fn CLI(&mut self, operation: &Operation) {
        todo!();
    }

    fn CLV(&mut self, operation: &Operation) {
        todo!();
    }

    fn CMP(&mut self, operation: &Operation) {
        todo!();
    }

    fn CPX(&mut self, operation: &Operation) {
        todo!();
    }

    fn CPY(&mut self, operation: &Operation) {
        todo!();
    }

    fn DEC(&mut self, operation: &Operation) {
        todo!();
    }

    fn DEX(&mut self, operation: &Operation) {
        todo!();
    }

    fn DEY(&mut self, operation: &Operation) {
        todo!();
    }

    fn EOR(&mut self, operation: &Operation) {
        todo!();
    }

    fn INC(&mut self, operation: &Operation) {
        todo!();
    }

    fn INX(&mut self, operation: &Operation) {
        todo!();
    }

    fn INY(&mut self, operation: &Operation) {
        todo!();
    }

    fn JMP(&mut self, operation: &Operation) {
        todo!();
    }

    fn JSR(&mut self, operation: &Operation) {
        todo!();
    }

    fn LDA(&mut self, operation: &Operation) {
        todo!();
    }

    fn LDX(&mut self, operation: &Operation) {
        todo!();
    }

    fn LDY(&mut self, operation: &Operation) {
        todo!();
    }

    fn LSR(&mut self, operation: &Operation) {
        todo!();
    }

    fn NOP(&mut self, operation: &Operation) {
        todo!();
    }

    fn ORA(&mut self, operation: &Operation) {
        todo!();
    }

    fn PHA(&mut self, operation: &Operation) {
        todo!();
    }

    fn PHP(&mut self, operation: &Operation) {
        todo!();
    }

    fn PLA(&mut self, operation: &Operation) {
        todo!();
    }

    fn PLP(&mut self, operation: &Operation) {
        todo!();
    }

    fn ROL(&mut self, operation: &Operation) {
        todo!();
    }

    fn ROR(&mut self, operation: &Operation) {
        todo!();
    }

    fn RTI(&mut self, operation: &Operation) {
        todo!();
    }

    fn RTS(&mut self, operation: &Operation) {
        todo!();
    }

    fn SBC(&mut self, operation: &Operation) {
        todo!();
    }

    fn SEC(&mut self, operation: &Operation) {
        todo!();
    }

    fn SED(&mut self, operation: &Operation) {
        todo!();
    }

    fn SEI(&mut self, operation: &Operation) {
        todo!();
    }

    fn STA(&mut self, operation: &Operation) {
        todo!();
    }

    fn STX(&mut self, operation: &Operation) {
        todo!();
    }

    fn STY(&mut self, operation: &Operation) {
        todo!();
    }

    fn TAX(&mut self, operation: &Operation) {
        todo!();
    }

    fn TAY(&mut self, operation: &Operation) {
        todo!();
    }

    fn TSX(&mut self, operation: &Operation) {
        todo!();
    }

    fn TXA(&mut self, operation: &Operation) {
        todo!();
    }

    fn TXS(&mut self, operation: &Operation) {
        todo!();
    }

    fn TYA(&mut self, operation: &Operation) {
        todo!();
    }

}
