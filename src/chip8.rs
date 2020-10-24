#[allow(dead_code)]
#[allow(non_snake_case)]
use memory::Memory;
use registers::Registers;

use keyboard::{Button, KeyPad};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{thread, time};

use minifb::{Key, Scale, Window, WindowOptions};
use rand;
use rand::Rng;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

enum PC {
    Next,
    Skip,
    Jump(usize),
}

impl PC {
    pub fn skip_if(condition: bool) -> PC {
        if condition {
            PC::Skip
        } else {
            PC::Next
        }
    }
}

pub struct Chip8 {
    window: Window,
    pub mem: Memory,
    pub reg: Registers,
    buffer: Vec<u32>,
    pub keyboard: KeyPad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            mem: Memory::new_with_fonts(&FONT),
            reg: Registers::new(),
            buffer: vec![0; 64 * 32],
            window: Window::new(
                "chust8",
                64,
                32,
                WindowOptions {
                    borderless: false,
                    title: true,
                    resize: false,
                    scale: Scale::X8,
                    ..WindowOptions::default()
                },
            )
            .unwrap(),
            keyboard: KeyPad::new(),
        }
    }

    pub fn open_file(&mut self, filepath: &str) {
        let path = Path::new(&filepath);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => panic!("Unable to open file: {}", filepath),
        };
        self.read_file(file);
    }

    pub fn read_file(&mut self, mut file: File) {
        let mut file_buffer: [u8; 3_584] = [0; 3_584];
        match file.read(&mut file_buffer) {
            Ok(len) => println!("filelen: {}", len),
            Err(_) => panic!("file read error"),
        };
        self.copy_rom(file_buffer);
    }

    pub fn copy_rom(&mut self, file_buffer: [u8; 3_584]) {
        let mut offset = 0x200;
        for &bytes in file_buffer.iter() {
            self.mem.memory[offset] = bytes;
            offset += 1;
        }
    }

    /*
    Dxyn - DRW Vx, Vy, nibble
    Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

    The interpreter reads n bytes from memory, starting at the address stored in I.
    These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    VF is set to 1, otherwise it is set to 0.
    If the sprite is positioned so part of it is outside the coordinates of the display,
    it wraps around to the opposite side of the screen.
    See instruction 8xy3 for more information on XOR, and section 2.4,
    Display, for more information on the Chip-8 screen and sprites.
     */
    pub fn draw(&mut self, x: usize, y: usize, n: usize) {
        let mem_start = self.reg.vI as usize;
        let _mem_end = (self.reg.vI + (n as u16)) as usize;
        let height = 32;
        let width = 64;
        let color = 0xff;

        let mut collision = false;
        for byte in 0..n {
            for bit in 0..8 {
                let y = (y as usize + byte) % height;
                let x = (x as usize + bit) % width;

                let pixel = bit + byte * 32;
                println!("pixel: {}", pixel);
                println!("x: {}", x);
                println!("y: {}", y);
                if (self.mem.memory[mem_start + byte] & (0x80 >> bit)) != 0x00 {
                    if self.buffer[pixel] == 0xFFFF_FFFF {
                        collision = true;
                    }
                    //self.buffer[pixel] ^ 0xFFFF_FFFF;
                }
                self.reg.v[0x0F] |= color & self.buffer[pixel] as u8;
                self.buffer[pixel] = if color == 1 { 0xFFFF_FFFF } else { 0x0 }
            }
        }

        if collision {
            self.reg.v[0x0F] = 0x1;
        } else {
            self.reg.v[0x0F] = 0x0;
        }
        //let mut buf: Vec<u32> = vec![0xffffffff; 64 * 32];
        self.window.update_with_buffer(&self.buffer, 2, 2).unwrap();
        //let ten = time::Duration::from_millis(1000);
        //thread::sleep(ten);
        //self.window.update();
    }

    pub fn read_instruction(&mut self) -> u16 {
        let pc = self.reg.program_counter;
        let bottom = self.mem.memory[pc as usize];
        let top = self.mem.memory[(pc + 1) as usize];
        let instruction = (bottom as u16) << 8 | (top as u16);

        println!(
            "bottom: {:X}, top: {:X}, pc: {:X}, inst: {:X}",
            bottom, top, pc, instruction
        );
        instruction
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.decode_instruction();
            //let ten = time::Duration::from_millis(1000);
            //thread::sleep(ten);
            //let _ = self.window.update_with_buffer(&self.buffer);
        }
    }

    fn op_0_0_e_0(&self) -> PC {
        println!("We should be clearing the screen here");
        PC::Next
    }

    fn op_0_0_e_e(&mut self) -> PC {
        self.reg.stack_pointer -= 1;
        let jump = self.reg.stack_pointer as usize;
        PC::Jump(jump)
    }

    fn op_1_n_n_n(&mut self, nnn: usize) -> PC {
        PC::Jump(nnn)
    }

    fn op_2_n_n_n(&mut self, nnn: usize) -> PC {
        self.reg.stack_pointer += 1;
        self.reg.stack[self.reg.stack_pointer as usize] = self.reg.program_counter;
        PC::Jump(nnn)
    }

    fn op_3_x_k_k(&self, x: usize, kk: usize) -> PC {
        if self.reg.v[x as usize] == kk as u8 {
            PC::Skip
        } else {
            PC::Next
        }
    }

    fn op_4_x_k_k(&self, x: usize, kk: usize) -> PC {
        if self.reg.v[x as usize] != kk as u8 {
            PC::Skip
        } else {
            PC::Next
        }
    }

    fn op_5_x_y_0(&self, x: usize, y: usize) -> PC {
        if self.reg.v[x as usize] == self.reg.v[y as usize] {
            PC::Skip
        } else {
            PC::Next
        }
    }

    fn op_6_x_k_k(&mut self, x: usize, kk: usize) -> PC {
        self.reg.v[x as usize] = kk as u8;
        PC::Next
    }

    fn op_7_x_k_k(&mut self, x: usize, kk: usize) -> PC {
        self.reg.v[x as usize] = self.reg.v[x as usize].wrapping_add(kk as u8);
        PC::Next
    }

    fn op_8_x_y_z(&mut self, x: usize, y: usize, z: usize) -> PC {
        match z as u8 {
            0x0 => self.reg.v[x] = self.reg.v[y],
            0x1 => self.reg.v[x] = self.reg.v[x] | self.reg.v[y],
            0x2 => self.reg.v[x] = self.reg.v[x] & self.reg.v[y],
            0x3 => self.reg.v[x] = self.reg.v[x] ^ self.reg.v[y],
            0x4 => {
                let vx = self.reg.v[x] as u16;
                let vy = self.reg.v[y] as u16;
                let add = vx + vy;
                self.reg.v[x] = add as u8;
                self.reg.v[0x0F] = if add > 255 { 1 } else { 0 }
            }
            0x5 => {
                let vx = self.reg.v[x] as u16;
                let vy = self.reg.v[y] as u16;
                self.reg.v[0x0F] = if vx > vy { 1 } else { 0 };
                self.reg.v[x] = self.reg.v[x].wrapping_sub(self.reg.v[y])
            }
            0x6 => {
                self.reg.v[0xF] = self.reg.v[x] & 1;
                self.reg.v[x] = self.reg.v[x] >> 1;
            }
            0x7 => {
                let vx = self.reg.v[x] as u16;
                let vy = self.reg.v[y] as u16;
                self.reg.v[0x0F] = if vy > vx { 1 } else { 0 };
                self.reg.v[x] = self.reg.v[y].wrapping_sub(self.reg.v[x])
            }
            0xe => {
                self.reg.v[0xF] = (self.reg.v[x] >> 7) & 1;
                self.reg.v[x] = self.reg.v[x] << 1;
            }
            _ => println!("FAKE NEWS UP INHERE"),
        };
        PC::Next
    }

    fn op_9_x_y_0(&self, x: usize, y: usize) -> PC {
        if self.reg.v[x as usize] != self.reg.v[y as usize] {
            PC::Skip
        } else {
            PC::Next
        }
    }

    fn op_a_n_n_n(&mut self, nnn: u16) -> PC {
        self.reg.vI = nnn;
        PC::Next
    }

    fn op_b_n_n_n(&mut self, nnn: u16) -> PC {
        PC::Jump(nnn.wrapping_add(self.reg.v[0x0] as u16) as usize)
    }

    fn op_c_x_k_k(&mut self, x: usize, kk: usize) -> PC {
        let mut rng = rand::thread_rng();
        let rand: u8 = rng.gen_range(0, 255);
        self.reg.v[x] = rand & kk as u8;
        PC::Next
    }

    fn op_d_x_y_z(&mut self, x: usize, y: usize, z: usize) -> PC {
        let pos_x = self.reg.v[x as usize] as usize;
        let pos_y = self.reg.v[y as usize] as usize;
        self.draw(pos_x, pos_y, z);
        PC::Next
    }

    fn op_e_x_9_e(&mut self, x: usize) -> PC {
        let button = Button::from_u8(self.reg.v[x]);
        PC::skip_if(self.keyboard.is_button_pressed(button))
    }

    fn op_e_x_a_1(&mut self, x: usize) -> PC {
        let button = Button::from_u8(self.reg.v[x]);
        PC::skip_if(!self.keyboard.is_button_pressed(button))
    }

    // Match on the first 4 bit of the instruction
    // Does not advance the program counter
    pub fn decode_instruction(&mut self) {
        let instruction = self.read_instruction();
        let bits = (
            (instruction & 0xF000) >> 12 as u8,
            (instruction & 0x0F00) >> 8 as u8,
            (instruction & 0x00F0) >> 4 as u8,
            (instruction & 0x000F) as u8,
        );

        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        let z = (instruction & 0x000F) as usize;
        let kk = instruction & 0x00FF;
        let nnn = instruction & 0x0FFF;

        let pc_next = match bits {
            (0x0, 0x0, 0xE, 0x0) => self.op_0_0_e_0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_0_0_e_e(),
            (0x1, _, _, _) => self.op_1_n_n_n(nnn as usize),
            (0x2, _, _, _) => self.op_2_n_n_n(nnn as usize),
            (0x3, _, _, _) => self.op_3_x_k_k(x as usize, kk as usize),
            (0x4, _, _, _) => self.op_4_x_k_k(x as usize, kk as usize),
            (0x5, _, _, 0x0) => self.op_5_x_y_0(x as usize, y as usize),
            (0x6, _, _, _) => self.op_6_x_k_k(x as usize, kk as usize),
            (0x7, _, _, _) => self.op_7_x_k_k(x as usize, kk as usize),
            (0x8, _, _, _) => self.op_8_x_y_z(x as usize, y as usize, z),
            (0x9, _, _, _) => self.op_9_x_y_0(x as usize, y as usize),
            (0xA, _, _, _) => self.op_a_n_n_n(nnn),
            (0xB, _, _, _) => self.op_b_n_n_n(nnn),
            (0xC, _, _, _) => self.op_c_x_k_k(x as usize, kk as usize),
            (0xD, _, _, _) => self.op_d_x_y_z(x as usize, y as usize, z),
            (0xE, _, 0x9, 0xE) => self.op_e_x_9_e(x as usize),
            (0xE, _, 0xA, 0x1) => self.op_e_x_a_1(x as usize),
            _ => {
                println!("the end is neigh");
                PC::Next
            }
        };

        match pc_next {
            PC::Next => self.reg.program_counter += 2,
            PC::Skip => self.reg.program_counter += 4,
            PC::Jump(new_pc) => self.reg.program_counter = new_pc as u16,
        }
    }
}
