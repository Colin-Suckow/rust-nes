//http://nesdev.com/6502_cpu.txt
struct 6502 {
  //Registers
  PC: u16, //Program counter
  S: u8, //Stack pointer
  P: u8, //Processor status
  A: u8, //Accumulator
  X: u8, //Index X
  Y: u8, //Index Y
}

