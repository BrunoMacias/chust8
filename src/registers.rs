#[allow(dead_code)]
#[allow(non_snake_case)]

pub struct Registers {
    pub v: [u8; 16],

    pub vI: u16,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub program_counter: u16,

    pub stack_pointer: u8,
    pub stack: [u16; 16],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            v: [0; 16],

            vI: 0,

            delay_timer: 0,
            sound_timer: 0,

            program_counter: 0x200,

            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
