pub struct Joypad {
    buttons: u8,
    state: u8
}

impl Joypad {
    pub fn new() -> Joypad {
        let joypad = Joypad {
            buttons: 0,
            state: 0
        };
        joypad
    }

    pub fn read(&self) -> u8 {
        self.buttons
    }

    pub fn write(&mut self, inputs: u8) {
        self.buttons = inputs;
    }

    pub fn shift(&mut self) {
        self.state <<= 1;
    }

    pub fn set_state(&mut self) {
        self.state = self.buttons;
    }

    pub fn read_state(&mut self) -> u8 {
        self.state
    }
}