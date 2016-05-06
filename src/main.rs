pub mod chip8;
pub mod memory;
pub mod registers;

use chip8::Chip8;

fn main() {
    println!("Hello, world");
    let cpu = Chip8::new();
}
