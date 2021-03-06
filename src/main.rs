extern crate minifb;
extern crate rand;
extern crate ringbuffer;

pub mod chip8;
pub mod keyboard;
pub mod memory;
pub mod registers;

use chip8::Chip8;

use std::env;

fn main() {
    let filename: &str = &env::args().nth(1).unwrap()[..];
    let mut cpu = Chip8::new();

    cpu.open_file(filename);

    cpu.run();
}
