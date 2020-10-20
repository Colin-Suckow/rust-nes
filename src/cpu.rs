use crate::instruction::{AddressingMode, Instruction, Operation, OPCODES};
use crate::memory::*;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

//http://nesdev.com/6502_cpu.txt
pub struct Cpu<T: AddressSpace> {
    bus: T,
    //Registers
    PC: u16, //Program counter
    S: u16,   //Stack pointer
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

    pub fn step_cycle(&mut self) {
        //skip cycle if instruction is still in progress
        if self.operation_progress > 0 {
            self.operation_progress -= 1;
            return;
        }

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
            Instruction::JSR => self.JSR(u16::from_le_bytes([operation.data[0], operation.data[1]])),
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

    pub fn fetch_operand(&self, operation: &Operation) -> MemoryRead {
        match operation.addressing_mode {
            AddressingMode::Accumulator => MemoryRead {
                result: self.A,
                cross_page: false,
            },
            AddressingMode::Immediate => MemoryRead {
                result: operation.data[0],
                cross_page: false,
            },

            AddressingMode::Implied => MemoryRead {
                //TODO make None
                result: 0,
                cross_page: false,
            },
            AddressingMode::Relative => MemoryRead {
                result: self.bus.peek(relative_address(operation.data[0], self.PC)),
                cross_page: (operation.data[0] / 255) as u16 != (self.PC / 255) as u16,
            },
            AddressingMode::Absolute => MemoryRead {
                result: self.bus.peek(absolute_address(operation.data[0], operation.data[1])),
                cross_page: false,
            }, //Be careful of endianess here
            AddressingMode::ZeroPage => MemoryRead {
                result: self.bus.peek(zero_page_address(operation.data[0])),
                cross_page: false,
            },
            AddressingMode::Indirect => MemoryRead {
                result: self
                    .bus
                    .peek(u16::from_le_bytes([operation.data[0], operation.data[1]]) + self.PC)
                    ,
                cross_page: (u16::from_le_bytes([operation.data[0], operation.data[1]]) / 255) as u16
                    != (self.PC / 255) as u16,
            },
            AddressingMode::AbsoluteX => MemoryRead {
                result: self
                    .bus
                    .peek(u16::from_le_bytes([operation.data[0], operation.data[1]]) + self.X as u16)
                    ,
                cross_page: (u16::from_le_bytes([operation.data[0], operation.data[1]]) / 255) as u16
                    != (self.X / 255) as u16,
            },
            AddressingMode::AbsoluteY => MemoryRead {
                result: self
                    .bus
                    .peek(u16::from_le_bytes([operation.data[0], operation.data[1]]) + self.Y as u16)
                    ,
                cross_page: (u16::from_le_bytes([operation.data[0], operation.data[1]]) / 255) as u16
                    != (self.Y / 255) as u16,
            },

            AddressingMode::ZeroPageX => MemoryRead {
                result: self.bus.peek(operation.data[0].wrapping_add(self.X) as u16),
                cross_page: (operation.data[0] as u16 / 255) != (self.X / 255) as u16,
            },
            AddressingMode::ZeroPageY => MemoryRead {
                result: self.bus.peek(operation.data[0].wrapping_add(self.Y) as u16),
                cross_page: (operation.data[0] as u16 / 255) != (self.Y / 255) as u16,
            },

            AddressingMode::IndirectX => {
                let addressLocation = operation.data[0].wrapping_add(self.X);
                MemoryRead {
                    result: self
                        .bus
                        .peek(self.bus.peek_16(addressLocation as u16))
                        ,
                    cross_page: false,
                }
            }

            AddressingMode::IndirectY => {
                let address = self.bus.peek(operation.data[0] as u16) + self.Y;
                MemoryRead {
                    result: self.bus.peek(address as u16),
                    cross_page: false,
                }
            }
        }
    }

    fn consume_next_operation(&mut self) -> Operation {
        print!("PC: {} ", self.PC);
        let opcode_byte = self.bus.peek(self.PC);
        println!("Byte: {}", opcode_byte);
        self.PC += 1;
        let mut operation = OPCODES[opcode_byte as usize].clone().unwrap();
        let extra_bytes: u16 = match operation.addressing_mode {
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Indirect => 2,
            AddressingMode::Implied => 0,
            _ => 1
        };
        for i in (0..extra_bytes) {
            operation.data.push(self.bus.peek(self.PC + i));
        };
        self.PC += extra_bytes;
        operation
    }

    //CPU functions

    fn ADC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn AND(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn ASL(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BCC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BCS(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BEQ(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BIT(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BMI(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BNE(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BPL(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BRK(&mut self, operand: &MemoryRead) -> Option<u8> {
        //TODO Interrupts
        Some(5)
    }

    fn BVC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn BVS(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CLC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CLD(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CLI(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CLV(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CMP(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CPX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn CPY(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn DEC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn DEX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn DEY(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn EOR(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn INC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn INX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn INY(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn JMP(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn JSR(&mut self, operand: u16) -> Option<u8> {
        self.push_16(self.PC);
        self.PC = operand;
        Some(4)
    }

    fn LDA(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn LDX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn LDY(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn LSR(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn NOP(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn ORA(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn PHA(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn PHP(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn PLA(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn PLP(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn ROL(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn ROR(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn RTI(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn RTS(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn SBC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn SEC(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn SED(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn SEI(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn STA(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn STX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn STY(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn TAX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn TAY(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn TSX(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn TXA(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn TXS(&mut self, operand: &MemoryRead) -> Option<u8> {
        todo!();
    }

    fn TYA(&mut self, operand: &MemoryRead) -> Option<u8> {
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
        operation.data = vec![10];
        cpu.PC = 10;
        let value = cpu.fetch_operand(&operation);
        assert_eq!(value.result, 20);
        assert_eq!(value.cross_page, false);
    }

    #[test]
    fn test_operand_Absolute() {
        let mut cpu = Cpu::new(TestBus);
        let mut operation = crate::instruction::OPCODES[0x20].clone().unwrap();
        operation.data = vec![0x10, 0x20];
        cpu.PC = 10;
        let value = cpu.fetch_operand(&operation);
        assert_eq!(value.result, (0x2010 % 255) as u8);
        assert_eq!(value.cross_page, false);
    }
}
