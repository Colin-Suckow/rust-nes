use crate::cpu::{AddressingMode, AddressingMode::*, Instruction, Instruction::*};

#[allow(dead_code)]
pub static OPCODES: [Option<(Instruction, AddressingMode)>; 256] = [
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
    None,
    //0x82
    None,
    //0x83
    None,
    //0x84
    None,
    //0x85
    None,
    //0x86
    None,
    //0x87
    None,
    //0x88
    None,
    //0x89
    None,
    //0x8a
    None,
    //0x8b
    None,
    //0x8c
    None,
    //0x8d
    None,
    //0x8e
    None,
    //0x8f
    None,
    //0x90
    None,
    //0x91
    None,
    //0x92
    None,
    //0x93
    None,
    //0x94
    None,
    //0x95
    None,
    //0x96
    None,
    //0x97
    None,
    //0x98
    None,
    //0x99
    None,
    //0x9a
    None,
    //0x9b
    None,
    //0x9c
    None,
    //0x9d
    None,
    //0x9e
    None,
    //0x9f
    None,
    //0xa0
    None,
    //0xa1
    None,
    //0xa2
    None,
    //0xa3
    None,
    //0xa4
    None,
    //0xa5
    None,
    //0xa6
    None,
    //0xa7
    None,
    //0xa8
    None,
    //0xa9
    None,
    //0xaa
    None,
    //0xab
    None,
    //0xac
    None,
    //0xad
    None,
    //0xae
    None,
    //0xaf
    None,
    //0xb0
    None,
    //0xb1
    None,
    //0xb2
    None,
    //0xb3
    None,
    //0xb4
    None,
    //0xb5
    None,
    //0xb6
    None,
    //0xb7
    None,
    //0xb8
    None,
    //0xb9
    None,
    //0xba
    None,
    //0xbb
    None,
    //0xbc
    None,
    //0xbd
    None,
    //0xbe
    None,
    //0xbf
    None,
    //0xc0
    None,
    //0xc1
    None,
    //0xc2
    None,
    //0xc3
    None,
    //0xc4
    None,
    //0xc5
    None,
    //0xc6
    None,
    //0xc7
    None,
    //0xc8
    None,
    //0xc9
    None,
    //0xca
    None,
    //0xcb
    None,
    //0xcc
    None,
    //0xcd
    None,
    //0xce
    None,
    //0xcf
    None,
    //0xd0
    None,
    //0xd1
    None,
    //0xd2
    None,
    //0xd3
    None,
    //0xd4
    None,
    //0xd5
    None,
    //0xd6
    None,
    //0xd7
    None,
    //0xd8
    None,
    //0xd9
    None,
    //0xda
    None,
    //0xdb
    None,
    //0xdc
    None,
    //0xdd
    None,
    //0xde
    None,
    //0xdf
    None,
    //0xe0
    None,
    //0xe1
    None,
    //0xe2
    None,
    //0xe3
    None,
    //0xe4
    None,
    //0xe5
    None,
    //0xe6
    None,
    //0xe7
    None,
    //0xe8
    None,
    //0xe9
    None,
    //0xea
    None,
    //0xeb
    None,
    //0xec
    None,
    //0xed
    None,
    //0xee
    None,
    //0xef
    None,
    //0xf0
    None,
    //0xf1
    None,
    //0xf2
    None,
    //0xf3
    None,
    //0xf4
    None,
    //0xf5
    None,
    //0xf6
    None,
    //0xf7
    None,
    //0xf8
    None,
    //0xf9
    None,
    //0xfa
    None,
    //0xfb
    None,
    //0xfc
    None,
    //0xfd
    None,
    //0xfe
    None,
    //0xff
    None,
];

//Helper function to make opcode declarations easier
const fn oc(instruction: Instruction, addressing_mode: AddressingMode) -> Option<(Instruction, AddressingMode)> {
    Some((instruction, addressing_mode))
}
