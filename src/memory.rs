pub struct Memory {
    pub  memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Memory { memory: [0; 4096] }
    }
    pub fn new_with_fonts(fonts: &[u8]) -> Self {
        let mut mem = Memory::new();
        let mut offset = 0x50;
        for &byte in fonts {
            mem.memory[offset] = byte;
            offset += 1;
        }
        mem
    }
}
