pub mod chip8;
pub mod memory;
pub mod registers;

use chip8::Chip8;

use std::env;

fn main() {
    println!("Hello, world");
    let filename: &str = &env::args().nth(1).unwrap()[..];
    println!("Filename: {}",filename);
    
    let mut cpu = Chip8::new();
    
    
    cpu.open_file(filename);
}
