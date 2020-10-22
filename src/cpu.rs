use crate::instruction::{AddressingMode, Instruction, Operation, OPCODES};
use crate::memory::*;
use bit_field::BitField;

pub enum Operand {
    Constant { value: u8 },
    Address { location: u16 },
    Accumulator,
    None,
}

//http://nesdev.com/6502_cpu.txt
pub struct Cpu<T: AddressSpace> {
    bus: T,
    //Registers
    PC: u16, //Program counter
    S: u16,  //Stack pointer
    P: u8,   //Processor status
    A: u8,   //Accumulator
    X: u8,   //Index X
    Y: u8,   //Index Y
    operation_progress: u8,
}

pub struct MemoryRead {
    result: u8,
    cross_page: bool,
}

impl<T: AddressSpace> Cpu<T> {
    pub fn new(bus: T) -> Cpu<T> {
        Cpu {
            bus: bus,
            PC: 0,
            S: 0x0100,
            P: 0,
            A: 0,
            X: 0,
            Y: 0,
            operation_progress: 0,
        }
    }

    pub fn reset(&mut self) {
        self.PC = self.bus.peek_16(0xFFFC);
    }

    fn push(&mut self, value: u8) {
        self.bus.poke(self.S, value);
        self.S += 1;
    }

    fn pop(&mut self) -> u8 {
        let value = self.bus.peek(self.S);
        if self.S > 0x0100 {
            self.S -= 1;
        }
        value
    }

    fn push_16(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push((value & 0x00001111) as u8);
    }

    fn pop_16(&mut self) -> u16 {
        let byte2 = self.pop();
        let byte1 = self.pop();
        u16::from_le_bytes([byte1, byte2])
    }

    fn set_Z(&mut self, val: bool) {
        self.P.set_bit(1, val);
    }

    fn get_Z(&self) -> bool {
        self.P.get_bit(1)
    }

    fn set_N(&mut self, val: bool) {
        self.P.set_bit(7, val);
    }

    fn set_B(&mut self, val: bool) {
        self.P.set_bit(4, val);
    }

    fn set_I(&mut self, val: bool) {
        self.P.set_bit(2, val);
    }

    pub fn step_cycle(&mut self) {
        //skip cycle if instruction is still in progress
        if self.operation_progress > 0 {
            self.operation_progress -= 1;
            return;
        }
        println!("A:{} X:{} Y:{}", self.A, self.X, self.Y);

        let operation = self.consume_next_operation();

        //Set and subtract one from operation length to count the current cycle
        self.operation_progress = operation.base_cycle_count - 1;

        let operand = self.fetch_operand(&operation);

        let extra_cycles = match operation.instruction {
            Instruction::ADC => self.ADC(&operand),
            Instruction::AND => self.AND(&operand),
            Instruction::ASL => self.ASL(&operand),
            Instruction::BCC => self.BCC(&operand),
            Instruction::BCS => self.BCS(&operand),
            Instruction::BEQ => self.BEQ(&operand),
            Instruction::BIT => self.BIT(&operand),
            Instruction::BMI => self.BMI(&operand),
            Instruction::BNE => self.BNE(&operand),
            Instruction::BPL => self.BPL(&operand),
            Instruction::BRK => self.BRK(&operand),
            Instruction::BVC => self.BVC(&operand),
            Instruction::BVS => self.BVS(&operand),
            Instruction::CLC => self.CLC(&operand),
            Instruction::CLD => self.CLD(&operand),
            Instruction::CLI => self.CLI(&operand),
            Instruction::CLV => self.CLV(&operand),
            Instruction::CMP => self.CMP(&operand),
            Instruction::CPX => self.CPX(&operand),
            Instruction::CPY => self.CPY(&operand),
            Instruction::DEC => self.DEC(&operand),
            Instruction::DEX => self.DEX(&operand),
            Instruction::DEY => self.DEY(&operand),
            Instruction::EOR => self.EOR(&operand),
            Instruction::INC => self.INC(&operand),
            Instruction::INX => self.INX(&operand),
            Instruction::INY => self.INY(&operand),
            Instruction::JMP => self.JMP(&operand),
            Instruction::JSR => self.JSR(&operand),
            Instruction::LDA => self.LDA(&operand),
            Instruction::LDX => self.LDX(&operand),
            Instruction::LDY => self.LDY(&operand),
            Instruction::LSR => self.LSR(&operand),
            Instruction::NOP => self.NOP(&operand),
            Instruction::ORA => self.ORA(&operand),
            Instruction::PHA => self.PHA(&operand),
            Instruction::PHP => self.PHP(&operand),
            Instruction::PLA => self.PLA(&operand),
            Instruction::PLP => self.PLP(&operand),
            Instruction::ROL => self.ROL(&operand),
            Instruction::ROR => self.ROR(&operand),
            Instruction::RTI => self.RTI(&operand),
            Instruction::RTS => self.RTS(&operand),
            Instruction::SBC => self.SBC(&operand),
            Instruction::SEC => self.SEC(&operand),
            Instruction::SED => self.SED(&operand),
            Instruction::SEI => self.SEI(&operand),
            Instruction::STA => self.STA(&operand),
            Instruction::STX => self.STX(&operand),
            Instruction::STY => self.STY(&operand),
            Instruction::TAX => self.TAX(&operand),
            Instruction::TAY => self.TAY(&operand),
            Instruction::TSX => self.TSX(&operand),
            Instruction::TXA => self.TXA(&operand),
            Instruction::TXS => self.TXS(&operand),
            Instruction::TYA => self.TYA(&operand),
        };

        //Add extra cycles to the op length if the executing the instruction caused it
        if let Some(cycles) = extra_cycles {
            self.operation_progress += cycles;
        };
    }
    //(operation.data[0] / 255) as u16 != (self.PC / 255) as u16
    pub fn fetch_operand(&self, operation: &Operation) -> Operand {
        match operation.addressing_mode {
            AddressingMode::Accumulator => Operand::Accumulator,
            AddressingMode::Immediate => Operand::Constant {
                value: operation.data[0],
            },
            AddressingMode::Implied => Operand::None,
            AddressingMode::Relative => Operand::Address {
                location: relative_address(operation.data[0], self.PC),
            },
            AddressingMode::Absolute => Operand::Address {
                location: absolute_address(operation.data[0], operation.data[1]),
            },
            AddressingMode::ZeroPage => Operand::Address {
                location: zero_page_address(operation.data[0]),
            },
            AddressingMode::Indirect => Operand::Address {
                location: u16::from_le_bytes([operation.data[0], operation.data[1]]) + self.PC,
            },
            AddressingMode::AbsoluteX => Operand::Address {
                location: u16::from_le_bytes([operation.data[0], operation.data[1]])
                    + self.X as u16,
            },
            AddressingMode::AbsoluteY => Operand::Address {
                location: u16::from_le_bytes([operation.data[0], operation.data[1]])
                    + self.Y as u16,
            },

            AddressingMode::ZeroPageX => Operand::Address {
                location: operation.data[0].wrapping_add(self.X) as u16,
            },
            AddressingMode::ZeroPageY => Operand::Address {
                location: operation.data[0].wrapping_add(self.Y) as u16,
            },

            AddressingMode::IndirectX => {
                let addressLocation = operation.data[0].wrapping_add(self.X);
                Operand::Address {
                    location: self.bus.peek_16(addressLocation as u16),
                }
            }

            AddressingMode::IndirectY => {
                let address = self.bus.peek(operation.data[0] as u16) + self.Y;
                Operand::Address {
                    location: self.bus.peek_16(address as u16),
                }
            }
        }
    }

    fn consume_next_operation(&mut self) -> Operation {
        print!("PC: {:#X} ", self.PC);
        let opcode_byte = self.bus.peek(self.PC);
        println!("Byte: {:#X}", opcode_byte);
        self.PC += 1;
        let mut operation = OPCODES[opcode_byte as usize].clone().unwrap();
        let extra_bytes: u16 = match operation.addressing_mode {
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Indirect => 2,
            AddressingMode::Implied => 0,
            _ => 1,
        };
        for i in (0..extra_bytes) {
            operation.data.push(self.bus.peek(self.PC + i));
        }
        self.PC += extra_bytes;
        operation
    }

    //CPU functions

    fn ADC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn AND(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn ASL(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BCC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BCS(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BEQ(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_Z() {
            self.PC = addr;
            Some(2)
        } else {
            None
        }
    }

    fn BIT(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BMI(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BNE(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BPL(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BRK(&mut self, operand: &Operand) -> Option<u8> {
        self.PC += 1;
        self.set_I(true);
        self.set_B(true);
        let vec = self.bus.peek_16(0xFFFE);
        self.push_16(self.PC);
        self.push(self.P);
        self.PC = vec;
        Some(5)
    }

    fn BVC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn BVS(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CLC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CLD(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CLI(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CLV(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CMP(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CPX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn CPY(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn DEC(&mut self, operand: &Operand) -> Option<u8> {
        let address = unpack_address(operand);
        let val = self.bus.peek(address).wrapping_sub(1);
        self.bus.poke(address, val);
        self.set_Z(val == 0);
        self.set_N(val.get_bit(7));
        Some(2)
    }

    fn DEX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn DEY(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn EOR(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn INC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn INX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn INY(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn JMP(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn JSR(&mut self, operand: &Operand) -> Option<u8> {
        self.push_16(self.PC);
        self.PC = unpack_address(operand);
        Some(4)
    }

    fn LDA(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => {self.A = value.clone()}
            Operand::Address { location } => {self.A = self.bus.peek(location.clone())}
            _ => ()
        }
        self.set_N(self.A.get_bit(7));
        self.set_Z(self.A == 0);
        None
    }

    fn LDX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn LDY(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.Y = value.clone(),
            Operand::Address { location } => self.Y = self.bus.peek(location.clone()),
            Operand::Accumulator => self.Y = self.A,
            Operand::None => (),
        }

        self.set_N(self.Y.get_bit(7));
        self.set_Z(self.Y == 0);

        None
    }

    fn LSR(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn NOP(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn ORA(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn PHA(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn PHP(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn PLA(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn PLP(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn ROL(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn ROR(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn RTI(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn RTS(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
        self.PC = self.pop_16() + 1;
        None
    }

    fn SBC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn SEC(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn SED(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn SEI(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn STA(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(location.clone(), self.A),
            _ => {}
        }
        None
    }

    fn STX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn STY(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn TAX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn TAY(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn TSX(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn TXA(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn TXS(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }

    fn TYA(&mut self, operand: &Operand) -> Option<u8> {
        todo!();
    }
}

fn unpack_address(operand: &Operand) -> u16 {
    match operand {
        Operand::Address { location } => location.clone(),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{Bus, Ram};
}
