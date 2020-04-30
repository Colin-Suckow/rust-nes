

//http://nesdev.com/6502_cpu.txt
pub struct Cpu {
  //Registers
  PC: u16, //Program counter
  S: u8, //Stack pointer
  P: u8, //Processor status
  A: u8, //Accumulator
  X: u8, //Index X
  Y: u8, //Index Y
}

pub enum AddressingMode {
  Accumulator,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Immediate,
  Implied,
  Indirect,
  IndirectX,
  IndirectY,
  Relative,
  ZeroPage,
  ZeroPageX,
  ZeroPageY
}

pub enum Instruction {
  ADC,
  AND,
  ASL,
  BCC,
  BCS,
  BEQ,
  BIT,
  BMI,
  BNE,
  BPL,
  BRK,
  BVC,
  BVS,
  CLC,
  CLD,
  CLI,
  CLV,
  CMP,
  CPX,
  CPY,
  DEC,
  DEX,
  DEY,
  EOR,
  INC,
  INX,
  INY,
  JMP,
  JSR,
  LDA,
  LDX,
  LDY,
  LSR,
  NOP,
  ORA,
  PHA,
  PHP,
  PLA,
  PLP,
  ROL,
  ROR,
  RTI,
  RTS,
  SBC,
  SEC,
  SED,
  SEI,
  STA,
  STX,
  STY,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
}

