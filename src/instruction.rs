use crate::instruction::AddressingMode::*;
use crate::instruction::Instruction::*;

#[derive(Debug, Clone, Copy)]
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
    ZeroPageY,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Operation {
    pub instruction: Instruction,
    pub addressing_mode: AddressingMode,
    pub base_cycle_count: u8,
    pub data: Option<Vec<u8>>,
}


#[allow(dead_code)]
pub static OPCODES: [Option<Operation>; 256] = [
    //0x00
    oc(BRK, Implied),
    //0x01
    oc(ORA, Indirect),
    //0x02
    None,
    //0x03
    None,
    //0x04
    None,
    //0x05
    oc(ORA, ZeroPage),
    //0x06
    oc(ASL, ZeroPage),
    //0x07
    None,
    //0x08
    oc(PHP, Implied),
    //0x09
    oc(ORA, Immediate),
    //0x0a
    oc(ASL, Accumulator),
    //0x0b
    None,
    //0x0c
    None,
    //0x0d
    oc(ORA, Absolute),
    //0x0e
    oc(ASL, Absolute),
    //0x0f
    None,
    //0x10
    oc(BPL, Relative),
    //0x11
    oc(ORA, IndirectY),
    //0x12
    None,
    //0x13
    None,
    //0x14
    None,
    //0x15
    oc(ORA, ZeroPageX),
    //0x16
    oc(ASL, ZeroPageX),
    //0x17
    None,
    //0x18
    oc(CLC, Implied),
    //0x19
    oc(ORA, AbsoluteY),
    //0x1a
    None,
    //0x1b
    None,
    //0x1c
    None,
    //0x1d
    oc(ORA, AbsoluteX),
    //0x1e
    oc(ASL, AbsoluteX),
    //0x1f
    None,
    //0x20
    oc(JSR, Absolute),
    //0x21
    oc(AND, IndirectX),
    //0x22
    None,
    //0x23
    None,
    //0x24
    oc(BIT, ZeroPage),
    //0x25
    oc(AND, ZeroPage),
    //0x26
    oc(ROL, ZeroPage),
    //0x27
    None,
    //0x28
    oc(PLP, Implied),
    //0x29
    oc(AND, Immediate),
    //0x2a
    oc(ROL, Accumulator),
    //0x2b
    None,
    //0x2c
    oc(BIT, Absolute),
    //0x2d
    oc(AND, Absolute),
    //0x2e
    oc(ROL, Absolute),
    //0x2f
    None,
    //0x30
    oc(BMI, Relative),
    //0x31
    oc(AND, IndirectX),
    //0x32
    None,
    //0x33
    None,
    //0x34
    None,
    //0x35
    oc(AND, ZeroPageX),
    //0x36
    oc(ROL, ZeroPageX),
    //0x37
    None,
    //0x38
    oc(SEC, Implied),
    //0x39
    oc(AND, AbsoluteY),
    //0x3a
    None,
    //0x3b
    None,
    //0x3c
    None,
    //0x3d
    oc(AND, AbsoluteX),
    //0x3e
    oc(ROL, AbsoluteX),
    //0x3f
    None,
    //0x40
    oc(RTI, Implied),
    //0x41
    oc(EOR, IndirectX),
    //0x42
    None,
    //0x43
    None,
    //0x44
    None,
    //0x45
    oc(EOR, ZeroPage),
    //0x46
    oc(LSR, ZeroPage),
    //0x47
    None,
    //0x48
    oc(PHA, Implied),
    //0x49
    oc(EOR, Immediate),
    //0x4a
    oc(LSR, Accumulator),
    //0x4b
    None,
    //0x4c
    oc(JMP, Absolute),
    //0x4d
    oc(EOR, Absolute),
    //0x4e
    oc(LSR, Absolute),
    //0x4f
    None,
    //0x50
    oc(BVC, Relative),
    //0x51
    oc(EOR, IndirectY),
    //0x52
    None,
    //0x53
    None,
    //0x54
    None,
    //0x55
    oc(EOR, ZeroPageX),
    //0x56
    oc(LSR, ZeroPageX),
    //0x57
    None,
    //0x58
    oc(CLI, Implied),
    //0x59
    oc(EOR, AbsoluteY),
    //0x5a
    None,
    //0x5b
    None,
    //0x5c
    None,
    //0x5d
    oc(EOR, AbsoluteX),
    //0x5e
    oc(LSR, AbsoluteX),
    //0x5f
    None,
    //0x60
    oc(RTS, Implied),
    //0x61
    oc(ADC, IndirectX),
    //0x62
    None,
    //0x63
    None,
    //0x64
    None,
    //0x65
    oc(ADC, ZeroPage),
    //0x66
    oc(ROR, ZeroPage),
    //0x67
    None,
    //0x68
    oc(PLA, Implied),
    //0x69
    oc(ADC, Immediate),
    //0x6a
    oc(ROR, Accumulator),
    //0x6b
    None,
    //0x6c
    oc(JMP, Indirect),
    //0x6d
    oc(ADC, Absolute),
    //0x6e
    oc(ROR, Absolute),
    //0x6f
    None,
    //0x70
    oc(BVS, Relative),
    //0x71
    oc(ADC, IndirectY),
    //0x72
    None,
    //0x73
    None,
    //0x74
    None,
    //0x75
    oc(ADC, ZeroPageX),
    //0x76
    oc(ROR, ZeroPageX),
    //0x77
    None,
    //0x78
    oc(SEI, Implied),
    //0x79
    oc(ADC, AbsoluteY),
    //0x7a
    None,
    //0x7b
    None,
    //0x7c
    None,
    //0x7d
    oc(ADC, AbsoluteX),
    //0x7e
    oc(ROR, AbsoluteX),
    //0x7f
    None,
    //0x80
    None,
    //0x81
    oc(STA, Indirect),
    //0x82
    None,
    //0x83
    None,
    //0x84
    oc(STY, ZeroPage),
    //0x85
    oc(STA, ZeroPage),
    //0x86
    oc(STX, ZeroPage),
    //0x87
    None,
    //0x88
    oc(DEY, Implied),
    //0x89
    None,
    //0x8a
    oc(TXA, Implied),
    //0x8b
    None,
    //0x8c
    oc(STY, Absolute),
    //0x8d
    oc(STA, Absolute),
    //0x8e
    oc(STX, Absolute),
    //0x8f
    None,
    //0x90
    oc(BCC, Relative),
    //0x91
    oc(STA, IndirectY),
    //0x92
    None,
    //0x93
    None,
    //0x94
    oc(STY, ZeroPageX),
    //0x95
    oc(STA, ZeroPageX),
    //0x96
    oc(STX, ZeroPageY),
    //0x97
    None,
    //0x98
    oc(TYA, Implied),
    //0x99
    oc(STA, AbsoluteY),
    //0x9a
    oc(TXS, Implied),
    //0x9b
    None,
    //0x9c
    None,
    //0x9d
    oc(STA, AbsoluteX),
    //0x9e
    None,
    //0x9f
    None,
    //0xa0
    oc(LDY, Immediate),
    //0xa1
    oc(LDA, IndirectX),
    //0xa2
    oc(LDX, Immediate),
    //0xa3
    None,
    //0xa4
    oc(LDY, ZeroPage),
    //0xa5
    oc(LDA, ZeroPage),
    //0xa6
    oc(LDX, ZeroPage),
    //0xa7
    None,
    //0xa8
    oc(TAY, Implied),
    //0xa9
    oc(LDA, Immediate),
    //0xaa
    oc(TAX, Implied),
    //0xab
    None,
    //0xac
    oc(LDY, Absolute),
    //0xad
    oc(LDA, Absolute),
    //0xae
    oc(LDX, Absolute),
    //0xaf
    None,
    //0xb0
    oc(BCS, Relative),
    //0xb1
    oc(LDA, IndirectY),
    //0xb2
    None,
    //0xb3
    None,
    //0xb4
    oc(LDY, ZeroPageX),
    //0xb5
    oc(LDA, ZeroPageX),
    //0xb6
    oc(LDX, ZeroPageY),
    //0xb7
    None,
    //0xb8
    oc(CLV, Implied),
    //0xb9
    oc(LDA, AbsoluteY),
    //0xba
    oc(TSX, Implied),
    //0xbb
    None,
    //0xbc
    oc(LDY, AbsoluteX),
    //0xbd
    oc(LDA, AbsoluteX),
    //0xbe
    oc(LDX, AbsoluteY),
    //0xbf
    None,
    //0xc0
    oc(CPY, Immediate),
    //0xc1
    oc(CMP, IndirectX),
    //0xc2
    None,
    //0xc3
    None,
    //0xc4
    oc(CPY, ZeroPage),
    //0xc5
    oc(CMP, ZeroPage),
    //0xc6
    oc(DEC, ZeroPage),
    //0xc7
    None,
    //0xc8
    oc(INY, Implied),
    //0xc9
    oc(CMP, Immediate),
    //0xca
    oc(DEX, Implied),
    //0xcb
    None,
    //0xcc
    oc(CPY, Absolute),
    //0xcd
    oc(CMP, Absolute),
    //0xce
    oc(DEC, Absolute),
    //0xcf
    None,
    //0xd0
    oc(BNE, Relative),
    //0xd1
    oc(CMP, IndirectY),
    //0xd2
    None,
    //0xd3
    None,
    //0xd4
    None,
    //0xd5
    oc(CMP, ZeroPageX),
    //0xd6
    oc(DEC, ZeroPageX),
    //0xd7
    None,
    //0xd8
    oc(CLD, Implied),
    //0xd9
    oc(CMP, AbsoluteY),
    //0xda
    None,
    //0xdb
    None,
    //0xdc
    None,
    //0xdd
    oc(CMP, AbsoluteX),
    //0xde
    oc(DEC, AbsoluteX),
    //0xdf
    None,
    //0xe0
    oc(CPX, Immediate),
    //0xe1
    oc(SBC, IndirectX),
    //0xe2
    None,
    //0xe3
    None,
    //0xe4
    oc(CPX, ZeroPage),
    //0xe5
    oc(SBC, ZeroPage),
    //0xe6
    oc(INC, ZeroPage),
    //0xe7
    None,
    //0xe8
    oc(INX, Implied),
    //0xe9
    oc(SBC, Immediate),
    //0xea
    oc(NOP, Implied),
    //0xeb
    None,
    //0xec
    oc(CPX, Absolute),
    //0xed
    oc(SBC, Absolute),
    //0xee
    oc(INC, Absolute),
    //0xef
    None,
    //0xf0
    oc(BEQ, Relative),
    //0xf1
    oc(SBC, IndirectY),
    //0xf2
    None,
    //0xf3
    None,
    //0xf4
    None,
    //0xf5
    oc(SBC, ZeroPageX),
    //0xf6
    oc(INC, ZeroPageX),
    //0xf7
    None,
    //0xf8
    oc(SED, Implied),
    //0xf9
    oc(SBC, AbsoluteY),
    //0xfa
    None,
    //0xfb
    None,
    //0xfc
    None,
    //0xfd
    oc(SBC, AbsoluteX),
    //0xfe
    oc(INC, AbsoluteX),
    //0xff
    None,
];

//Helper function to make opcode declarations easier
const fn oc(
    instruction: Instruction,
    addressing_mode: AddressingMode,
) -> Option<Operation> {

    let base_cycle_count = match addressing_mode {
        ZeroPageX => 4,
        ZeroPageY => 4,
        AbsoluteX => 4,
        AbsoluteY => 4,
        IndirectX => 6,
        IndirectY => 5,
        _ => 2,
    };


    Some(Operation {
        instruction,
        addressing_mode,
        base_cycle_count,
        data: None,
    })
}
