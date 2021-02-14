#[allow(non_snake_case)]

use minifb::{Key};

/*
1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F
 */

pub enum Button {
    One,
    Two,
    Three,
    Four,
    Q,
    W,
    E,
    R,
    A,
    S,
    D,
    F,
    Z,
    X,
    C,
    V,
}

impl Button {
    pub fn from_key(key: Key) -> Button {
        match key {
            Key::Key0 => Button::One,
            Key::Key1 => Button::Two,
            Key::Key2 => Button::Three,
            Key::Key3 => Button::Four,
            Key::Q => Button::Q,
            Key::W => Button::W,
            Key::E => Button::E,
            Key::R => Button::R,
            Key::A => Button::A,
            Key::S => Button::S,
            Key::D => Button::D,
            Key::F => Button::F,
            Key::Z => Button::Z,
            Key::X => Button::X,
            Key::C => Button::C,
            Key::V => Button::V,
            // Probably shouldn't actually panic
            _ => panic!("unknown key pressed"),
        }

    }

    pub fn from_u8(key: u8) -> Button {
        match key {
            0 => Button::One,
            1 => Button::Two,
            2 => Button::Three,
            3 => Button::Four,
            4 => Button::Q,
            5 => Button::W,
            6 => Button::E,
            7 => Button::R,
            8 => Button::A,
            9 => Button::S,
            10 => Button::D,
            11 => Button::F,
            12 => Button::Z,
            13 => Button::X,
            14 => Button::C,
            15 => Button::V,
            // Probably shouldn't actually panic
            _ => panic!("unknown key pressed"),
        }
    }

    pub fn to_u8(button: Button) -> u8 {
        match button {
            Button::One => 0,
            Button::Two => 1,
            Button::Three => 2,
            Button::Four => 3,
            Button::Q => 4,
            Button::W => 5,
            Button::E => 6,
            Button::R => 7,
            Button::A => 8,
            Button::S => 9,
            Button::D => 10,
            Button::F => 11,
            Button::Z => 12,
            Button::X => 13,
            Button::C => 14,
            Button::V => 15
        }
    }
}

pub struct KeyPad {
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    q: bool,
    w: bool,
    e: bool,
    r: bool,
    a: bool,
    s: bool,
    d: bool,
    f: bool,
    z: bool,
    x: bool,
    c: bool,
    v: bool,
}

impl KeyPad {
    pub fn new() -> KeyPad {
        KeyPad {
            one: false,
            two: false,
            three: false,
            four: false,
            q: false,
            w: false,
            e: false,
            r: false,
            a: false,
            s: false,
            d: false,
            f: false,
            z: false,
            x: false,
            c: false,
            v: false,
        }
    }

    pub fn set_button_pressed(&mut self, button: Button, pressed: bool) {
        match button {
            Button::One => self.one = pressed,
            Button::Two => self.two = pressed,
            Button::Three => self.three = pressed,
            Button::Four => self.four = pressed,
            Button::Q => self.q = pressed,
            Button::W => self.w = pressed,
            Button::E => self.e = pressed,
            Button::R => self.r = pressed,
            Button::A => self.a = pressed,
            Button::S => self.s = pressed,
            Button::D => self.d = pressed,
            Button::F => self.f = pressed,
            Button::Z => self.z = pressed,
            Button::X => self.x = pressed,
            Button::C => self.c = pressed,
            Button::V => self.v = pressed,
        }
    }

    pub fn is_button_pressed(&mut self, button: Button) -> bool {
        match button {
            Button::One => self.one,
            Button::Two => self.two,
            Button::Three => self.three,
            Button::Four => self.four,
            Button::Q => self.q,
            Button::W => self.w,
            Button::E => self.e,
            Button::R => self.r,
            Button::A => self.a,
            Button::S => self.s,
            Button::D => self.d,
            Button::F => self.f,
            Button::Z => self.z,
            Button::X => self.x,
            Button::C => self.c,
            Button::V => self.v,
        }
    }
}
