mod memory;

use memory::AddressSpace;

fn main() {
    println!("Hello, world!");
    let mut ram = memory::Ram::new(2048);
    ram.poke(0, 15);
    println!("Byte 0: {}", ram.peek(3000));
}
