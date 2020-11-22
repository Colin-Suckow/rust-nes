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
    pc: u16, //Program counter
    s: u8,   //Stack pointer
    p: u8,   //Processor status
    a: u8,   //Accumulator
    x: u8,   //Index X
    y: u8,   //Index Y
    operation_progress: u8,
}

impl<T: AddressSpace> Cpu<T> {
    pub fn new(bus: T) -> Cpu<T> {
        Cpu {
            bus,
            pc: 0,
            s: 0x0FD,
            p: 0x24,
            a: 0,
            x: 0,
            y: 0,
            operation_progress: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = self.bus.peek_16(0xFFFC); //0xC000 for nestest
        self.p = 0x24;
    }

    fn push(&mut self, value: u8) {
        //println!("-->push {:#X}", value);
        self.bus.poke(self.s as u16 + 0x100, value);
        self.s -= 1;
    }

    fn pop(&mut self) -> u8 {
        self.s += 1;
        let value = self.bus.peek(self.s as u16 + 0x100);
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

    fn set_z(&mut self, val: bool) {
        self.p.set_bit(1, val);
    }

    fn get_z(&self) -> bool {
        self.p.get_bit(1)
    }

    fn set_n(&mut self, val: bool) {
        self.p.set_bit(7, val);
    }

    fn get_n(&self) -> bool {
        self.p.get_bit(7)
    }

    fn set_b(&mut self, val: bool) {
        self.p.set_bit(4, val);
    }

    fn set_i(&mut self, val: bool) {
        self.p.set_bit(2, val);
    }

    fn set_c(&mut self, val: bool) {
        self.p.set_bit(0, val);
    }

    fn get_c(&self) -> bool {
        self.p.get_bit(0)
    }

    fn set_v(&mut self, val: bool) {
        self.p.set_bit(6, val);
    }

    fn get_v(&self) -> bool {
        self.p.get_bit(6)
    }

    fn set_d(&mut self, val: bool) {
        self.p.set_bit(3, val);
    }

    pub fn fire_nmi(&mut self) {
        let addr = self.bus.peek_16(0xFFFA);
        self.push_16(self.pc);
        self.push(self.p);
        self.pc = addr;
    }

    pub fn step_cycle(&mut self) {
        //skip cycle if instruction is still in progress
        if self.operation_progress > 0 {
            self.operation_progress -= 1;
            return;
        }

        let _inst_pc = self.pc;

        let operation = self.consume_next_operation();

        //Set and subtract one from operation length to count the current cycle
        self.operation_progress = operation.base_cycle_count - 1;

        let operand = self.fetch_operand(&operation);

        let _operand_value = match &operand {
            Operand::Constant { value } => *value as u16,
            Operand::Address { location } => *location,
            Operand::Accumulator => self.a as u16,
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
            Instruction::ADC => self.adc(&operand),
            Instruction::AND => self.and(&operand),
            Instruction::ASL => self.asl(&operand),
            Instruction::BCC => self.bcc(&operand),
            Instruction::BCS => self.bcs(&operand),
            Instruction::BEQ => self.beq(&operand),
            Instruction::BIT => self.bit(&operand),
            Instruction::BMI => self.bmi(&operand),
            Instruction::BNE => self.bne(&operand),
            Instruction::BPL => self.bpl(&operand),
            Instruction::BRK => self.brk(&operand),
            Instruction::BVC => self.bvc(&operand),
            Instruction::BVS => self.bvs(&operand),
            Instruction::CLC => self.clc(&operand),
            Instruction::CLD => self.cld(&operand),
            Instruction::CLI => self.cli(&operand),
            Instruction::CLV => self.clv(&operand),
            Instruction::CMP => self.cmp(&operand),
            Instruction::CPX => self.cpx(&operand),
            Instruction::CPY => self.cpy(&operand),
            Instruction::DEC => self.dec(&operand),
            Instruction::DEX => self.dex(&operand),
            Instruction::DEY => self.dey(&operand),
            Instruction::EOR => self.eor(&operand),
            Instruction::INC => self.inc(&operand),
            Instruction::INX => self.inx(&operand),
            Instruction::INY => self.iny(&operand),
            Instruction::JMP => self.jmp(&operand),
            Instruction::JSR => self.jsr(&operand),
            Instruction::LDA => self.lda(&operand),
            Instruction::LDX => self.ldx(&operand),
            Instruction::LDY => self.ldy(&operand),
            Instruction::LSR => self.lsr(&operand),
            Instruction::NOP => self.nop(&operand),
            Instruction::ORA => self.ora(&operand),
            Instruction::PHA => self.pha(&operand),
            Instruction::PHP => self.php(&operand),
            Instruction::PLA => self.pla(&operand),
            Instruction::PLP => self.plp(&operand),
            Instruction::ROL => self.rol(&operand),
            Instruction::ROR => self.ror(&operand),
            Instruction::RTI => self.rti(&operand),
            Instruction::RTS => self.rts(&operand),
            Instruction::SBC => self.sbc(&operand),
            Instruction::SEC => self.sec(&operand),
            Instruction::SED => self.sed(&operand),
            Instruction::SEI => self.sei(&operand),
            Instruction::STA => self.sta(&operand),
            Instruction::STX => self.stx(&operand),
            Instruction::STY => self.sty(&operand),
            Instruction::TAX => self.tax(&operand),
            Instruction::TAY => self.tay(&operand),
            Instruction::TSX => self.tsx(&operand),
            Instruction::TXA => self.txa(&operand),
            Instruction::TXS => self.txs(&operand),
            Instruction::TYA => self.tya(&operand),
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
                location: relative_address(operation.data[0], self.pc),
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
                    + self.x as u16,
            },
            AddressingMode::AbsoluteY => Operand::Address {
                location: u16::from_le_bytes([operation.data[0], operation.data[1]])
                    .wrapping_add(self.y as u16),
            },

            AddressingMode::ZeroPageX => Operand::Address {
                location: operation.data[0].wrapping_add(self.x) as u16,
            },
            AddressingMode::ZeroPageY => Operand::Address {
                location: operation.data[0].wrapping_add(self.y) as u16,
            },

            AddressingMode::IndirectX => {
                let address_location = operation.data[0].wrapping_add(self.x);
                let byte1 = self.bus.peek(address_location as u16);
                let byte2 = match address_location {
                    0xFF => self.bus.peek(0),
                    _ => self.bus.peek((address_location + 1) as u16),
                };
                Operand::Address {
                    location: u16::from_le_bytes([byte1, byte2]),
                }
            }

            AddressingMode::IndirectY => {
                let address_location = operation.data[0];
                let byte1 = self.bus.peek(address_location as u16);
                let byte2 = match address_location {
                    0xFF => self.bus.peek(0),
                    _ => self.bus.peek((address_location + 1) as u16),
                };
                let address = u16::from_le_bytes([byte1, byte2]);

                Operand::Address {
                    location: address.wrapping_add(self.y as u16),
                }
            }
        }
    }

    fn consume_next_operation(&mut self) -> Operation {
        let opcode_byte = self.bus.peek(self.pc);
        //println!("PC: {:#X} : {:#X}", self.PC, opcode_byte);
        self.pc += 1;
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
            operation.data.push(self.bus.peek(self.pc + i));
        }
        self.pc += extra_bytes;
        operation
    }

    //CPU functions

    fn adc(&mut self, operand: &Operand) -> Option<u8> {
        let val = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        let old_c = self.get_c() as u8;
        let old_n = self.a.get_bit(7);
        self.set_c(self.a.checked_add(val.saturating_add(old_c)) == None);
        self.a = self.a.wrapping_add(val.wrapping_add(old_c));
        self.set_n(self.a.get_bit(7));
        self.set_v(old_n == val.get_bit(7) && old_n != self.a.get_bit(7));
        self.set_z(self.a == 0);
        None
    }

    fn and(&mut self, operand: &Operand) -> Option<u8> {
        let op = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        self.a &= op;
        self.set_standard_flags(&self.a.clone());
        None
    }

    fn asl(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let shifted_val = val << 1;
                self.set_c(val.get_bit(7));
                self.set_n(shifted_val.get_bit(7));
                self.set_z(shifted_val == 0);
                self.bus.poke(*location, shifted_val);
            }
            Operand::Accumulator => {
                let val = self.a;
                let shifted_val = val << 1;
                self.set_c(val.get_bit(7));
                self.set_n(shifted_val.get_bit(7));
                self.set_z(shifted_val == 0);
                self.a = shifted_val;
                //self.PC -= 1;
            }
            _ => (),
        }

        None
    }

    fn bcc(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_c() {
            self.pc = addr;
            Some(2)
        } else {
            None
        }
    }

    fn bcs(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_c() {
            self.pc = addr;
            Some(2)
        } else {
            None
        }
    }

    fn beq(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_z() {
            self.pc = addr;
            Some(2)
        } else {
            None
        }
    }

    //11100100 nes
    //10100100 mine

    fn bit(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        let val = self.bus.peek(addr);
        self.set_n(val.get_bit(7));
        self.set_v(val.get_bit(6));
        self.set_z(val & self.a == 0);
        None
    }

    fn bmi(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_n() {
            self.pc = addr;
            Some(2)
        } else {
            None
        }
    }

    fn bne(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_z() {
            self.pc = addr;
            Some(2)
        } else {
            None
        }
    }

    fn bpl(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_n() {
            self.pc = addr;
        };
        None
    }

    fn brk(&mut self, _operand: &Operand) -> Option<u8> {
        self.pc += 1;
        self.set_i(true);
        self.set_b(true);
        let vec = self.bus.peek_16(0xFFFE);
        self.push_16(self.pc);
        self.pc = vec;
        Some(5)
    }

    fn bvc(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if !self.get_v() {
            self.pc = addr;
        };
        None
    }

    fn bvs(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        if self.get_v() {
            self.pc = addr;
        };
        None
    }

    fn clc(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_c(false);
        None
    }

    fn cld(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_d(false);
        None
    }

    fn cli(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_i(false);
        None
    }

    fn clv(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_v(false);
        None
    }

    fn cmp(&mut self, operand: &Operand) -> Option<u8> {
        let value = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            Operand::Accumulator => self.a,
            Operand::None => 0,
        };
        self.set_c(value <= self.a);
        self.set_n((self.a.wrapping_sub(value)).get_bit(7));
        self.set_z(value == self.a);
        None
    }

    fn cpx(&mut self, operand: &Operand) -> Option<u8> {
        let value = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            Operand::Accumulator => self.a,
            Operand::None => 0,
        };
        self.set_c(value <= self.x);
        self.set_n((self.x.wrapping_sub(value)).get_bit(7));
        self.set_z(value == self.x);
        None
    }

    fn cpy(&mut self, operand: &Operand) -> Option<u8> {
        let value = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            Operand::Accumulator => self.a,
            Operand::None => 0,
        };
        self.set_c(value <= self.y);
        self.set_n((self.y.wrapping_sub(value)).get_bit(7));
        self.set_z(value == self.y);
        None
    }

    fn dec(&mut self, operand: &Operand) -> Option<u8> {
        let address = unpack_address(operand);
        let val = self.bus.peek(address).wrapping_sub(1);
        self.bus.poke(address, val);
        self.set_z(val == 0);
        self.set_n(val.get_bit(7));
        Some(2)
    }

    fn dex(&mut self, _operand: &Operand) -> Option<u8> {
        self.x = self.x.wrapping_sub(1);
        self.set_standard_flags(&self.x.clone());
        None
    }

    fn dey(&mut self, _operand: &Operand) -> Option<u8> {
        self.y = self.y.wrapping_sub(1);
        self.set_standard_flags(&self.y.clone());
        None
    }

    fn eor(&mut self, operand: &Operand) -> Option<u8> {
        let op = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        self.a ^= op;
        self.set_standard_flags(&self.a.clone());
        None
    }

    fn inc(&mut self, operand: &Operand) -> Option<u8> {
        let address = unpack_address(operand);
        let val = self.bus.peek(address).wrapping_add(1);
        self.bus.poke(address, val);
        self.set_z(val == 0);
        self.set_n(val.get_bit(7));
        Some(2)
    }

    fn inx(&mut self, _operand: &Operand) -> Option<u8> {
        self.x = self.x.wrapping_add(1);
        self.set_standard_flags(&self.x.clone());
        None
    }

    fn iny(&mut self, _operand: &Operand) -> Option<u8> {
        self.y = self.y.wrapping_add(1);
        self.set_standard_flags(&self.y.clone());
        None
    }

    fn jmp(&mut self, operand: &Operand) -> Option<u8> {
        let addr = unpack_address(operand);
        self.pc = addr;
        None
    }

    fn jsr(&mut self, operand: &Operand) -> Option<u8> {
        self.push_16(self.pc - 1);
        self.pc = unpack_address(operand);
        Some(4)
    }

    fn lda(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.a = *value,
            Operand::Address { location } => self.a = self.bus.peek(*location),
            _ => (),
        };
        self.set_n(self.a.get_bit(7));
        self.set_z(self.a == 0);
        None
    }

    fn ldx(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.x = *value,
            Operand::Address { location } => self.x = self.bus.peek(*location),
            Operand::Accumulator => self.x = self.a,
            Operand::None => (),
        }

        self.set_n(self.x.get_bit(7));
        self.set_z(self.x == 0);

        None
    }

    fn ldy(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Constant { value } => self.y = *value,
            Operand::Address { location } => self.y = self.bus.peek(*location),
            Operand::Accumulator => self.y = self.a,
            Operand::None => (),
        }

        self.set_n(self.y.get_bit(7));
        self.set_z(self.y == 0);

        None
    }

    fn lsr(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let shifted_val = val >> 1;
                self.set_c(val.get_bit(0));
                self.set_n(false);
                self.set_z(shifted_val == 0);
                self.bus.poke(*location, shifted_val);
            }
            Operand::Accumulator => {
                let val = self.a;
                let shifted_val = val >> 1;
                self.set_c(val.get_bit(0));
                self.set_n(false);
                self.set_z(shifted_val == 0);
                self.a = shifted_val;
                //self.PC -= 1;
            }
            _ => (),
        }

        None
    }

    fn nop(&mut self, _operand: &Operand) -> Option<u8> {
        None
    }

    fn ora(&mut self, operand: &Operand) -> Option<u8> {
        let op = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        self.a |= op;
        self.set_standard_flags(&self.a.clone());
        None
    }

    fn pha(&mut self, _operand: &Operand) -> Option<u8> {
        self.push(self.a);
        Some(1)
    }

    fn php(&mut self, _operand: &Operand) -> Option<u8> {
        self.push(self.p | 0x10);
        Some(1)
    }

    fn pla(&mut self, _operand: &Operand) -> Option<u8> {
        self.a = self.pop();
        self.set_standard_flags(&self.a.clone());
        Some(2)
    }

    fn plp(&mut self, _operand: &Operand) -> Option<u8> {
        self.p = self.pop();
        self.p.set_bit(4, false);
        self.p.set_bit(5, true);
        Some(2)
    }

    fn rol(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let mut rotated_val = val << 1;
                rotated_val.set_bit(0, self.get_c());
                self.set_c(val.get_bit(7));
                self.set_n(rotated_val.get_bit(7));
                self.set_z(rotated_val == 0);
                self.bus.poke(*location, rotated_val);
            }
            Operand::Accumulator => {
                let val = self.a;
                let mut rotated_val = val << 1;
                rotated_val.set_bit(0, self.get_c());
                self.set_c(val.get_bit(7));
                self.set_n(rotated_val.get_bit(7));
                self.set_z(rotated_val == 0);
                self.a = rotated_val;
            }
            _ => (),
        }
        None
    }

    fn ror(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => {
                let val = self.bus.peek(*location);
                let mut rotated_val = val >> 1;
                rotated_val.set_bit(7, self.get_c());
                self.set_c(val.get_bit(0));
                self.set_n(rotated_val.get_bit(7));
                self.set_z(rotated_val == 0);
                self.bus.poke(*location, rotated_val);
            }
            Operand::Accumulator => {
                let val = self.a;
                let mut rotated_val = val >> 1;
                rotated_val.set_bit(7, self.get_c());
                self.set_c(val.get_bit(0));
                self.set_n(rotated_val.get_bit(7));
                self.set_z(rotated_val == 0);
                self.a = rotated_val;
                //self.PC -= 1;
            }
            _ => (),
        }

        None
    }

    fn rti(&mut self, _operand: &Operand) -> Option<u8> {
        self.p = self.pop();
        self.pc = self.pop_16();
        self.p.set_bit(4, false);
        self.p.set_bit(5, true);
        None
    }

    fn rts(&mut self, _operand: &Operand) -> Option<u8> {
        self.pc = self.pop_16().wrapping_add(1);
        None
    }

    fn sbc(&mut self, operand: &Operand) -> Option<u8> {
        let val = match operand {
            Operand::Constant { value } => *value,
            Operand::Address { location } => self.bus.peek(*location),
            _ => 0,
        };
        let carry = !self.get_c() as u8;
        let old_n = !self.a.get_bit(7);
        self.set_c(self.a.checked_sub(val.saturating_add(carry)) != None);
        self.a = self.a.wrapping_sub(val.wrapping_add(carry));
        self.set_n(self.a.get_bit(7));
        self.set_v(old_n == val.get_bit(7) && old_n == self.a.get_bit(7));
        self.set_z(self.a == 0);
        None
    }

    fn sec(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_c(true);
        None
    }

    fn sed(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_d(true);
        None
    }

    fn sei(&mut self, _operand: &Operand) -> Option<u8> {
        self.set_i(true);
        None
    }

    fn sta(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(*location, self.a),
            _ => {}
        }
        None
    }

    fn stx(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(*location, self.x),
            _ => {}
        }
        None
    }

    fn sty(&mut self, operand: &Operand) -> Option<u8> {
        match operand {
            Operand::Address { location } => self.bus.poke(*location, self.y),
            _ => {}
        }
        None
    }

    fn tax(&mut self, _operand: &Operand) -> Option<u8> {
        self.x = self.a;
        self.set_standard_flags(&self.x.clone());
        None
    }

    fn tay(&mut self, _operand: &Operand) -> Option<u8> {
        self.y = self.a;
        self.set_standard_flags(&self.y.clone());
        None
    }

    fn tsx(&mut self, _operand: &Operand) -> Option<u8> {
        self.x = self.s;
        self.set_standard_flags(&self.x.clone());
        None
    }

    fn txa(&mut self, _operand: &Operand) -> Option<u8> {
        self.a = self.x;
        self.set_standard_flags(&self.a.clone());
        None
    }

    fn txs(&mut self, _operand: &Operand) -> Option<u8> {
        self.s = self.x;
        None
    }

    fn tya(&mut self, _operand: &Operand) -> Option<u8> {
        self.a = self.y;
        self.set_standard_flags(&self.a.clone());
        None
    }
    fn set_standard_flags(&mut self, val: &u8) {
        self.set_z(*val == 0);
        self.set_n(val.get_bit(7));
    }
}

fn unpack_address(operand: &Operand) -> u16 {
    match operand {
        Operand::Address { location } => *location,
        _ => panic!(),
    }
}
