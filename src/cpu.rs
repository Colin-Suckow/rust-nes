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
    pub bus: T,
    //Registers
    PC: u16, //Program counter
    S: u8,   //Stack pointer
    P: u8,   //Processor status
    A: u8,   //Accumulator
    X: u8,   //Index X
    Y: u8,   //Index Y
    operation_progress: u8
}

impl<T: AddressSpace> Cpu<T> {
    pub fn new(bus: T) -> Cpu<T> {
        Cpu {
            bus,
            PC: 0,
            S: 0x0FD,
            P: 0x24,
            A: 0,
            X: 0,
            Y: 0,
            operation_progress: 0
        }
    }

    pub fn reset(&mut self) {
        self.PC = self.bus.peek_16(0xFFFC); //0xC000 for nestest
        self.P = 0x24;
    }

    fn push(&mut self, value: u8) {
        //println!("-->push {:#X}", value);
        self.bus.poke(self.S as u16 + 0x100, value);
        self.S -= 1;
    }

    fn pop(&mut self) -> u8 {
        self.S += 1;
        let value = self.bus.peek(self.S as u16 + 0x100);
        //println!("-->pop {:#X}", value);
        value
    }

    fn push_16(&mut self, value: u16) {
        //println!("push16 {:#X}", value);
        let bytes = value.to_le_bytes();
        self.push(bytes[1]);
        self.push(bytes[0]);
    }

    fn pop_16(&mut self) -> u16 {
        let byte1 = self.pop();
        let byte2 = self.pop();
        let num = u16::from_le_bytes([byte1, byte2]);
        //println!("pop16 {:#X}", num);
        num
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

    fn get_N(&self) -> bool {
        self.P.get_bit(7)
    }

    fn set_B(&mut self, val: bool) {
        self.P.set_bit(4, val);
    }

    fn set_I(&mut self, val: bool) {
        self.P.set_bit(2, val);
    }

    fn set_C(&mut self, val: bool) {
        self.P.set_bit(0, val);
    }

    fn get_C(&self) -> bool {
        self.P.get_bit(0)
    }

    fn set_V(&mut self, val: bool) {
        self.P.set_bit(6, val);
    }

    fn get_V(&self) -> bool {
        self.P.get_bit(6)
    }

    fn set_D(&mut self, val: bool) {
        self.P.set_bit(3, val);
    }

    fn get_D(&self) -> bool {
        self.P.get_bit(3)
    }

    pub fn fire_nmi(&mut self) {
        let addr = self.bus.peek_16(0xFFFA);
        self.push_16(self.PC);
        self.push(self.P);
        self.PC = addr;
    }

    pub fn step_cycle(&mut self) {
        //skip cycle if instruction is still in progress
        if self.operation_progress > 0 {
            self.operation_progress -= 1;
            return;
        }

        let _inst_PC = self.PC;

        let operation = self.consume_next_operation();

        //Set and subtract one from operation length to count the current cycle
        self.operation_progress = operation.base_cycle_count - 1;

        let operand = self.fetch_operand(&operation);

        let _operand_value = match &operand {
            Operand::Constant { value } => *value as u16,
            Operand::Address { location } => *location,
            Operand::Accumulator => self.A as u16,
            Operand::None => 0,
        };

        // let op_text = match &operand {
        //     Operand::Constant { value } => {
        //         format!("{:?} #${:02X}", operation.instruction, value.clone())
        //     }
        //     Operand::Address { location } => match &operation.addressing_mode {
        //         AddressingMode::Relative | AddressingMode::Absolute => {
        //             format!("{:?} ${:04X}", operation.instruction, location.clone())
        //         }
        //         _ => format!(
        //             "{:?} ${:02X} = {:02X}",
        //             operation.instruction,
        //             location.clone(),
        //             self.bus.peek(location.clone())
        //         ),
        //     },
        //     _ => format!("{:?}", operation.instruction),
        // };

        //Logging formatting
        // self.log.write_all(
        //     format!(
        //         "{:04X} {:31} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}\n",
        //         inst_PC, op_text, self.A, self.X, self.Y, self.P, self.S
        //     )
        //     .as_bytes(),
        // );

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
        //println!("{:#X}", self.bus.controller.status);
    }
    //(operation.data[0] / 255) as u16 != (self.PC / 255) as u16
    pub fn fetch_operand(&mut self, operation: &Operation) -> Operand {
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
            AddressingMode::Indirect => {
                let addr_location = u16::from_le_bytes([operation.data[0], operation.data[1]]);
                let byte1 = self.bus.peek(addr_location);
                let byte2 = match addr_location.to_le_bytes() {
                    [0xFF, hbyte] => {
                        let lbyte = self.bus.peek(0x00);
                        self.bus.peek(u16::from_le_bytes([lbyte, hbyte]))
                    }
                    _ => self.bus.peek(addr_location + 1),
                };
                Operand::Address {
                    location: u16::from_le_bytes([byte1, byte2]),
                }
            }
            AddressingMode::AbsoluteX => Operand::Address {
                location: u16::from_le_bytes([operation.data[0], operation.data[1]])
                    + self.X as u16,
            },
            AddressingMode::AbsoluteY => Operand::Address {
                location: u16::from_le_bytes([operation.data[0], operation.data[1]])
                    .wrapping_add(self.Y as u16),
            },

            AddressingMode::ZeroPageX => Operand::Address {
                location: operation.data[0].wrapping_add(self.X) as u16,
            },
            AddressingMode::ZeroPageY => Operand::Address {
                location: operation.data[0].wrapping_add(self.Y) as u16,
            },

            AddressingMode::IndirectX => {
                let addressLocation = operation.data[0].wrapping_add(self.X);
                let byte1 = self.bus.peek(addressLocation as u16);
                let byte2 = match addressLocation {
                    0xFF => self.bus.peek(0),
                    _ => self.bus.peek((addressLocation + 1) as u16),
                };
                Operand::Address {
                    location: u16::from_le_bytes([byte1, byte2]),
                }
            }

            AddressingMode::IndirectY => {
                let addressLocation = operation.data[0];
                let byte1 = self.bus.peek(addressLocation as u16);
                let byte2 = match addressLocation {
                    0xFF => self.bus.peek(0),
                    _ => self.bus.peek((addressLocation + 1) as u16),
                };
                let address = u16::from_le_bytes([byte1, byte2]);

                Operand::Address {
                    location: address.wrapping_add(self.Y as u16),
                }
            }
        }
    }

    fn consume_next_operation(&mut self) -> Operation {
        let opcode_byte = self.bus.peek(self.PC);
        //println!("PC: {:#X} : {:#X}", self.PC, opcode_byte);
        self.PC += 1;
        let mut operation = OPCODES[opcode_byte as usize]
            .clone()
            .unwrap_or_else(|| panic!("Unknown opcode {:#X}", opcode_byte));
        let extra_bytes: u16 = match operation.addressing_mode {
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Indirect => 2,
            AddressingMode::Implied => 0,
            AddressingMode::Accumulator => 0,
            _ => 1,
        };
        for i in 0..extra_bytes {
            operation.data.push(self.bus.peek(self.PC + i));
        }
        self.PC += extra_bytes;
        operation
    }

    //CPU functions

    fn ADC(&mut self, operand: &Operand) -> Option<u8> {
        let val = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        let old_c = self.get_C() as u8;
        let old_n = self.A.get_bit(7);
        self.set_C(self.A.checked_add(val.saturating_add(old_c)) == None);
        self.A = self.A.wrapping_add(val.wrapping_add(old_c));
        self.set_N(self.A.get_bit(7));
        self.set_V(old_n == val.get_bit(7) && old_n != self.A.get_bit(7));
        self.set_Z(self.A == 0);
        None
    }

    fn AND(&mut self, operand: &Operand) -> Option<u8> {
        let op = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        self.A &= op;
        self.set_standard_flags(&self.A.clone());
        None
    }

    fn ASL(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let shifted_val = val << 1;
                self.set_C(val.get_bit(7));
                self.set_N(shifted_val.get_bit(7));
                self.set_Z(shifted_val == 0);
                self.bus.poke(*location, shifted_val);
            }
            Operand::Accumulator => {
                let val = self.A;
                let shifted_val = val << 1;
                self.set_C(val.get_bit(7));
                self.set_N(shifted_val.get_bit(7));
                self.set_Z(shifted_val == 0);
                self.A = shifted_val;
                //self.PC -= 1;
            }
            _ => (),
        }

        None
    }

    fn BCC(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_C() {
            self.PC = addr;
            Some(2)
        } else {
            None
        }
    }

    fn BCS(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_C() {
            self.PC = addr;
            Some(2)
        } else {
            None
        }
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

    //11100100 nes
    //10100100 mine

    fn BIT(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        let val = self.bus.peek(addr);
        self.set_N(val.get_bit(7));
        self.set_V(val.get_bit(6));
        self.set_Z(val & self.A == 0);
        None
    }

    fn BMI(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_N() {
            self.PC = addr;
            Some(2)
        } else {
            None
        }
    }

    fn BNE(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_Z() {
            self.PC = addr;
            Some(2)
        } else {
            None
        }
    }

    fn BPL(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_N() {
            self.PC = addr;
        };
        None
    }

    fn BRK(&mut self, _operand: &Operand) -> Option<u8> {
        self.PC += 1;
        self.set_I(true);
        self.set_B(true);
        let vec = self.bus.peek_16(0xFFFE);
        self.push_16(self.PC);
        self.PC = vec;
        Some(5)
    }

    fn BVC(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_V() {
            self.PC = addr;
        };
        None
    }

    fn BVS(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_V() {
            self.PC = addr;
        };
        None
    }

    fn CLC(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_C(false);
        None
    }

    fn CLD(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_D(false);
        None
    }

    fn CLI(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_I(false);
        None
    }

    fn CLV(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_V(false);
        None
    }

    fn CMP(&mut self, operand: &Operand) -> Option<u8> {
        let value = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            Operand::Accumulator => self.A,
            Operand::None => 0,
        };
        self.set_C(value <= self.A);
        self.set_N((self.A.wrapping_sub(value)).get_bit(7));
        self.set_Z(value == self.A);
        None
    }

    fn CPX(&mut self, operand: &Operand) -> Option<u8> {
        let value = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            Operand::Accumulator => self.A,
            Operand::None => 0,
        };
        self.set_C(value <= self.X);
        self.set_N((self.X.wrapping_sub(value)).get_bit(7));
        self.set_Z(value == self.X);
        None
    }

    fn CPY(&mut self, operand: &Operand) -> Option<u8> {
        let value = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            Operand::Accumulator => self.A,
            Operand::None => 0,
        };
        self.set_C(value <= self.Y);
        self.set_N((self.Y.wrapping_sub(value)).get_bit(7));
        self.set_Z(value == self.Y);
        None
    }

    fn DEC(&mut self, operand: &Operand) -> Option<u8> {
        let address = unpack_address(operand);
        let val = self.bus.peek(address).wrapping_sub(1);
        self.bus.poke(address, val);
        self.set_Z(val == 0);
        self.set_N(val.get_bit(7));
        Some(2)
    }

    fn DEX(&mut self, _operand: &Operand) -> Option<u8> {
        self.X = self.X.wrapping_sub(1);
        self.set_standard_flags(&self.X.clone());
        None
    }

    fn DEY(&mut self, _operand: &Operand) -> Option<u8> {
        self.Y = self.Y.wrapping_sub(1);
        self.set_standard_flags(&self.Y.clone());
        None
    }

    fn EOR(&mut self, operand: &Operand) -> Option<u8> {
        let op = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        self.A ^= op;
        self.set_standard_flags(&self.A.clone());
        None
    }

    fn INC(&mut self, operand: &Operand) -> Option<u8> {
        let address = unpack_address(operand);
        let val = self.bus.peek(address).wrapping_add(1);
        self.bus.poke(address, val);
        self.set_Z(val == 0);
        self.set_N(val.get_bit(7));
        Some(2)
    }

    fn INX(&mut self, _operand: &Operand) -> Option<u8> {
        self.X = self.X.wrapping_add(1);
        self.set_standard_flags(&self.X.clone());
        None
    }

    fn INY(&mut self, _operand: &Operand) -> Option<u8> {
        self.Y = self.Y.wrapping_add(1);
        self.set_standard_flags(&self.Y.clone());
        None
    }

    fn JMP(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        self.PC = addr;
        None
    }

    fn JSR(&mut self, operand: &Operand) -> Option<u8> {
        self.push_16(self.PC - 1);
        self.PC = unpack_address(operand);
        Some(4)
    }

    fn LDA(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.A = *value,
            Operand::Address { location } => self.A = self.bus.peek(*location),
            _ => (),
        };
        self.set_N(self.A.get_bit(7));
        self.set_Z(self.A == 0);
        None
    }

    fn LDX(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.X = *value,
            Operand::Address { location } => self.X = self.bus.peek(*location),
            Operand::Accumulator => self.X = self.A,
            Operand::None => (),
        }

        self.set_N(self.X.get_bit(7));
        self.set_Z(self.X == 0);

        None
    }

    fn LDY(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.Y = *value,
            Operand::Address { location } => self.Y = self.bus.peek(*location),
            Operand::Accumulator => self.Y = self.A,
            Operand::None => (),
        }

        self.set_N(self.Y.get_bit(7));
        self.set_Z(self.Y == 0);

        None
    }

    fn LSR(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let shifted_val = val >> 1;
                self.set_C(val.get_bit(0));
                self.set_N(false);
                self.set_Z(shifted_val == 0);
                self.bus.poke(*location, shifted_val);
            }
            Operand::Accumulator => {
                let val = self.A;
                let shifted_val = val >> 1;
                self.set_C(val.get_bit(0));
                self.set_N(false);
                self.set_Z(shifted_val == 0);
                self.A = shifted_val;
                //self.PC -= 1;
            }
            _ => (),
        }

        None
    }

    fn NOP(&mut self, _operand: &Operand) -> Option<u8> {
        None
    }

    fn ORA(&mut self, operand: &Operand) -> Option<u8> {
        let op = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        self.A |= op;
        self.set_standard_flags(&self.A.clone());
        None
    }

    fn PHA(&mut self, _operand: &Operand) -> Option<u8> {
        self.push(self.A);
        Some(1)
    }

    fn PHP(&mut self, _operand: &Operand) -> Option<u8> {
        self.push(self.P | 0x10);
        Some(1)
    }

    fn PLA(&mut self, _operand: &Operand) -> Option<u8> {
        self.A = self.pop();
        self.set_standard_flags(&self.A.clone());
        Some(2)
    }

    fn PLP(&mut self, _operand: &Operand) -> Option<u8> {
        self.P = self.pop();
        self.P.set_bit(4, false);
        self.P.set_bit(5, true);
        Some(2)
    }

    fn ROL(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let mut rotated_val = val << 1;
                rotated_val.set_bit(0, self.get_C());
                self.set_C(val.get_bit(7));
                self.set_N(rotated_val.get_bit(7));
                self.set_Z(rotated_val == 0);
                self.bus.poke(*location, rotated_val);
            }
            Operand::Accumulator => {
                let val = self.A;
                let mut rotated_val = val << 1;
                rotated_val.set_bit(0, self.get_C());
                self.set_C(val.get_bit(7));
                self.set_N(rotated_val.get_bit(7));
                self.set_Z(rotated_val == 0);
                self.A = rotated_val;
            }
            _ => (),
        }
        None
    }

    fn ROR(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let mut rotated_val = val >> 1;
                rotated_val.set_bit(7, self.get_C());
                self.set_C(val.get_bit(0));
                self.set_N(rotated_val.get_bit(7));
                self.set_Z(rotated_val == 0);
                self.bus.poke(*location, rotated_val);
            }
            Operand::Accumulator => {
                let val = self.A;
                let mut rotated_val = val >> 1;
                rotated_val.set_bit(7, self.get_C());
                self.set_C(val.get_bit(0));
                self.set_N(rotated_val.get_bit(7));
                self.set_Z(rotated_val == 0);
                self.A = rotated_val;
                //self.PC -= 1;
            }
            _ => (),
        }

        None
    }

    fn RTI(&mut self, _operand: &Operand) -> Option<u8> {
        self.P = self.pop();
        self.PC = self.pop_16();
        self.P.set_bit(4, false);
        self.P.set_bit(5, true);
        None
    }

    fn RTS(&mut self, _operand: &Operand) -> Option<u8> {
        self.PC = self.pop_16().wrapping_add(1);
        None
    }

    fn SBC(&mut self, operand: &Operand) -> Option<u8> {
        let val = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        let carry = !self.get_C() as u8;
        let old_n = !self.A.get_bit(7);
        self.set_C(self.A.checked_sub(val.saturating_add(carry)) != None);
        self.A = self.A.wrapping_sub(val.wrapping_add(carry));
        self.set_N(self.A.get_bit(7));
        self.set_V(old_n == val.get_bit(7) && old_n == self.A.get_bit(7));
        self.set_Z(self.A == 0);
        None
    }

    fn SEC(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_C(true);
        None
    }

    fn SED(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_D(true);
        None
    }

    fn SEI(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_I(true);
        None
    }

    fn STA(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(*location, self.A),
            _ => {}
        }
        None
    }

    fn STX(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(*location, self.X),
            _ => {}
        }
        None
    }

    fn STY(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(*location, self.Y),
            _ => {}
        }
        None
    }

    fn TAX(&mut self, _operand: &Operand) -> Option<u8> {
        self.X = self.A;
        self.set_standard_flags(&self.X.clone());
        None
    }

    fn TAY(&mut self, _operand: &Operand) -> Option<u8> {
        self.Y = self.A;
        self.set_standard_flags(&self.Y.clone());
        None
    }

    fn TSX(&mut self, _operand: &Operand) -> Option<u8> {
        self.X = self.S;
        self.set_standard_flags(&self.X.clone());
        None
    }

    fn TXA(&mut self, _operand: &Operand) -> Option<u8> {
        self.A = self.X;
        self.set_standard_flags(&self.A.clone());
        None
    }

    fn TXS(&mut self, _operand: &Operand) -> Option<u8> {
        self.S = self.X;
        None
    }

    fn TYA(&mut self, _operand: &Operand) -> Option<u8> {
        self.A = self.Y;
        self.set_standard_flags(&self.A.clone());
        None
    }
    fn set_standard_flags(&mut self, val: &u8) {
        self.set_Z(*val == 0);
        self.set_N(val.get_bit(7));
    }
}

fn unpack_address(operand: &Operand) -> u16 {
    match operand {
        Operand::Address { location } => *location,
        _ => panic!(),
    }
}