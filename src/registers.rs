#[allow(dead_code)]
#[allow(non_snake_case)]

pub struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    vA: u8,
    vB: u8,
    vC: u8,
    vD: u8,
    vE: u8,
    vF: u8,

    vI: u16,

    delay_timer: u8,
    sound_timer: u8,

    program_counter: u16,

    stack_pointer: u8,
    stack: [u16; 16],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            vA: 0,
            vB: 0,
            vC: 0,
            vD: 0,
            vE: 0,
            vF: 0,

            vI: 0,

            delay_timer: 0,
            sound_timer: 0,

            program_counter: 0,

            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
