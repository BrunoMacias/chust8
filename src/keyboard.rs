#[allow(non_snake_case)]
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
            _ => panic!("unknown key pressed"),
        }
    }
}

pub struct KeyPad {
    One: bool,
    Two: bool,
    Three: bool,
    Four: bool,
    Q: bool,
    W: bool,
    E: bool,
    R: bool,
    A: bool,
    S: bool,
    D: bool,
    F: bool,
    Z: bool,
    X: bool,
    C: bool,
    V: bool,
}

impl KeyPad {
    pub fn new() -> KeyPad {
        KeyPad {
            One: false,
            Two: false,
            Three: false,
            Four: false,
            Q: false,
            W: false,
            E: false,
            R: false,
            A: false,
            S: false,
            D: false,
            F: false,
            Z: false,
            X: false,
            C: false,
            V: false,
        }
    }

    pub fn set_button_pressed(&mut self, button: Button, pressed: bool) {
        match button {
            Button::One => self.One = pressed,
            Button::Two => self.Two = pressed,
            Button::Three => self.Three = pressed,
            Button::Four => self.Four = pressed,
            Button::Q => self.Q = pressed,
            Button::W => self.W = pressed,
            Button::E => self.E = pressed,
            Button::R => self.R = pressed,
            Button::A => self.A = pressed,
            Button::S => self.S = pressed,
            Button::D => self.D = pressed,
            Button::F => self.F = pressed,
            Button::Z => self.Z = pressed,
            Button::X => self.X = pressed,
            Button::C => self.C = pressed,
            Button::V => self.V = pressed,
        }
    }

    pub fn is_button_pressed(&mut self, button: Button) -> bool {
        match button {
            Button::One => self.One,
            Button::Two => self.Two,
            Button::Three => self.Three,
            Button::Four => self.Four,
            Button::Q => self.Q,
            Button::W => self.W,
            Button::E => self.E,
            Button::R => self.R,
            Button::A => self.A,
            Button::S => self.S,
            Button::D => self.D,
            Button::F => self.F,
            Button::Z => self.Z,
            Button::X => self.X,
            Button::C => self.C,
            Button::V => self.V,
        }
    }
}
