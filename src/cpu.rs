use crate::instruction::{AddressingMode, Instruction, Operand, Operation};
use crate::memory::{AddressSpace, Bus, TestBus};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

//http://nesdev.com/6502_cpu.txt
pub struct Cpu<T: AddressSpace> {
    bus: T,
    //Registers
    PC: u16, //Program counter
    S: u8,   //Stack pointer
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

        //Set and subtract one from operation length to count the current cycle
        self.operation_progress = operation.base_cycle_count - 1;

        let extra_cycles = match operation.instruction {
            Instruction::ADC => self.ADC(&operation),
            Instruction::AND => self.AND(&operation),
            Instruction::ASL => self.ASL(&operation),
            Instruction::BCC => self.BCC(&operation),
            Instruction::BCS => self.BCS(&operation),
            Instruction::BEQ => self.BEQ(&operation),
            Instruction::BIT => self.BIT(&operation),
            Instruction::BMI => self.BMI(&operation),
            Instruction::BNE => self.BNE(&operation),
            Instruction::BPL => self.BPL(&operation),
            Instruction::BRK => self.BRK(&operation),
            Instruction::BVC => self.BVC(&operation),
            Instruction::BVS => self.BVS(&operation),
            Instruction::CLC => self.CLC(&operation),
            Instruction::CLD => self.CLD(&operation),
            Instruction::CLI => self.CLI(&operation),
            Instruction::CLV => self.CLV(&operation),
            Instruction::CMP => self.CMP(&operation),
            Instruction::CPX => self.CPX(&operation),
            Instruction::CPY => self.CPY(&operation),
            Instruction::DEC => self.DEC(&operation),
            Instruction::DEX => self.DEX(&operation),
            Instruction::DEY => self.DEY(&operation),
            Instruction::EOR => self.EOR(&operation),
            Instruction::INC => self.INC(&operation),
            Instruction::INX => self.INX(&operation),
            Instruction::INY => self.INY(&operation),
            Instruction::JMP => self.JMP(&operation),
            Instruction::JSR => self.JSR(&operation),
            Instruction::LDA => self.LDA(&operation),
            Instruction::LDX => self.LDX(&operation),
            Instruction::LDY => self.LDY(&operation),
            Instruction::LSR => self.LSR(&operation),
            Instruction::NOP => self.NOP(&operation),
            Instruction::ORA => self.ORA(&operation),
            Instruction::PHA => self.PHA(&operation),
            Instruction::PHP => self.PHP(&operation),
            Instruction::PLA => self.PLA(&operation),
            Instruction::PLP => self.PLP(&operation),
            Instruction::ROL => self.ROL(&operation),
            Instruction::ROR => self.ROR(&operation),
            Instruction::RTI => self.RTI(&operation),
            Instruction::RTS => self.RTS(&operation),
            Instruction::SBC => self.SBC(&operation),
            Instruction::SEC => self.SEC(&operation),
            Instruction::SED => self.SED(&operation),
            Instruction::SEI => self.SEI(&operation),
            Instruction::STA => self.STA(&operation),
            Instruction::STX => self.STX(&operation),
            Instruction::STY => self.STY(&operation),
            Instruction::TAX => self.TAX(&operation),
            Instruction::TAY => self.TAY(&operation),
            Instruction::TSX => self.TSX(&operation),
            Instruction::TXA => self.TXA(&operation),
            Instruction::TXS => self.TXS(&operation),
            Instruction::TYA => self.TYA(&operation),
        };

        //Add extra cycles to the op length if the executing the instruction caused it
        if let Some(cycles) = extra_cycles {
            self.operation_progress += cycles;
        };
    }

    pub fn fetch_operand(&self, operation: &Operation) -> MemoryRead {
        if let Some(data) = &operation.data {
            match operation.addressing_mode {
                AddressingMode::Accumulator => MemoryRead {
                    result: self.A,
                    cross_page: false,
                },
                AddressingMode::Immediate => MemoryRead {
                    result: data[1],
                    cross_page: false,
                },

                AddressingMode::Implied => MemoryRead { //TODO make None ((data[1] as i32) + self.PC as i32)
                    result: 0,
                    cross_page: false,
                }, 
                AddressingMode::Relative => MemoryRead {
                    result: self.bus.peek(15 as u16).unwrap(),
                    cross_page: (data[1] / 255) as u16 == (self.PC / 255) as u16,
                },
                AddressingMode::Absolute => MemoryRead {
                    result: self.bus.peek(15).unwrap(),
                    cross_page: false,
                }, //Be careful of endianess here
                AddressingMode::ZeroPage => MemoryRead {
                    result: self.bus.peek(data. as u16).unwrap(),
                    cross_page: false,
                },
                AddressingMode::Indirect => MemoryRead {
                    result: self
                        .bus
                        .peek((data[1] | (data[2] << 8)) as u16 + self.PC)
                        .unwrap(),
                    cross_page: ((data[1] | (data[2] << 8)) / 255) as u16 == (self.PC / 255) as u16,
                },

                AddressingMode::AbsoluteX => MemoryRead {
                    result: self
                        .bus
                        .peek((data[1] | (data[2] << 8)) as u16 + self.X as u16)
                        .unwrap(),
                    cross_page: ((data[1] | (data[2] << 8)) / 255) as u16 == (self.X / 255) as u16,
                },
                AddressingMode::AbsoluteY => MemoryRead {
                    result: self
                        .bus
                        .peek((data[1] | (data[2] << 8)) as u16 + self.Y as u16)
                        .unwrap(),
                    cross_page: ((data[1] | (data[2] << 8)) / 255) as u16 == (self.Y / 255) as u16,
                },

                AddressingMode::ZeroPageX => MemoryRead {
                    result: self.bus.peek(data[1].wrapping_add(self.X) as u16).unwrap(),
                    cross_page: (data[1] as u16 / 255) == (self.X / 255) as u16,
                },
                AddressingMode::ZeroPageY => MemoryRead {
                    result: self.bus.peek(data[1].wrapping_add(self.Y) as u16).unwrap(),
                    cross_page: (data[1] as u16 / 255) == (self.Y / 255) as u16,
                },

                AddressingMode::IndirectX => {
                    let address = data[1].wrapping_add(self.X);
                    MemoryRead {
                        result: self.bus.peek(address as u16).unwrap()
                            | (self.bus.peek((address + 1) as u16).unwrap() << 8),
                        cross_page: false,
                    }
                }

                AddressingMode::IndirectY => {
                    let address = data[1].wrapping_add(self.Y);
                    MemoryRead {
                        result: self.bus.peek((data[1] | (data[2] << 8)) as u16).unwrap(),
                        cross_page: false,
                    }
                }
            }
        } else {
            panic!("Unable to read opcode data");
        }
    }

    fn consume_next_operation(&mut self) -> Option<Operation> {
        todo!();
    }

    //CPU functions

    fn ADC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn AND(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn ASL(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BCC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BCS(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BEQ(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BIT(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BMI(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BNE(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BPL(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BRK(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BVC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn BVS(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CLC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CLD(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CLI(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CLV(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CMP(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CPX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn CPY(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn DEC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn DEX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn DEY(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn EOR(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn INC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn INX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn INY(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn JMP(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn JSR(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn LDA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn LDX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn LDY(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn LSR(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn NOP(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn ORA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn PHA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn PHP(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn PLA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn PLP(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn ROL(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn ROR(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn RTI(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn RTS(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn SBC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn SEC(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn SED(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn SEI(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn STA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn STX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn STY(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn TAX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn TAY(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn TSX(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn TXA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn TXS(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }

    fn TYA(&mut self, operation: &Operation) -> Option<u8> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{Bus, Ram};

    #[test]
    fn test_operand_relative() {
        let mut cpu = Cpu::new(TestBus);
        let mut operation = crate::instruction::OPCODES[0x10].clone().unwrap();
        operation.data = Some(vec![0xFF, 10]);
        cpu.PC = 10;
        let value = cpu.fetch_operand(&operation);
        assert_eq!(value.result, 20);
        assert_eq!(value.cross_page, false);
    }
}
