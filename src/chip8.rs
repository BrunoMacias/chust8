use memory::Memory;
use registers::Registers;

use std::path::Path;
use std::fs::File;
use std::io::Read;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

pub struct Chip8 {
    pub mem: Memory,
    pub reg: Registers,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            mem: Memory::new_with_fonts(&FONT),
            reg: Registers::new(),
        }
    }


    pub fn open_file(&mut self, filepath: &str){ 
        let path = Path::new(&filepath);
        let file = match File::open(&path){
            Ok(file) => file,
            Err(_) => panic!("Unable to open file: {}", filepath), 
        };
        self.read_file(file);
    }

    pub fn read_file(&mut self, mut file: File){
        let mut file_buffer: [u8; 3_584] = [0; 3_584];
        match file.read(&mut file_buffer){
            Ok(len) => println!("filelen: {}", len),
            Err(_) => panic!("file read error"),
        };
        self.copy_rom(file_buffer);
    }

    pub fn copy_rom(&mut self, mut file_buffer: [u8; 3_584]){ 
        let mut offset = 0x200;
        for &bytes in file_buffer.iter() {
            self.mem.memory[offset] = bytes;
            offset += 1;
        }
    }

}
