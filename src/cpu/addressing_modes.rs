use crate::cpu::CPU;

impl CPU {
    pub fn set_mode(&mut self, mode: Mode) -> u16 {
        let address: u16 = match mode {
            Mode::IMM => self.mode_imm(), 
            Mode::ZP => self.mode_zp(), 
            Mode::ZPX => self.mode_zpx(), 
            Mode::ZPY => self.mode_zpy(), 
            Mode::IZX => self.mode_izx(), 
            Mode::IZY => self.mode_izy(), 
            Mode::ABS => self.mode_abs(), 
            Mode::ABX => self.mode_abx(), 
            Mode::ABY => self.mode_aby(),
            Mode::ACC => self.mode_acc(),
            Mode::IND => self.mode_ind(), 
            Mode::REL => self.mode_rel(), 
            Mode::IMP => self.mode_imp()
        };
        address
    }

    // Absolute
    // 16-bit address is retrived from the next two bytes in the program counter
    pub fn mode_abs(&mut self) -> u16 {
        let first_byte = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        let second_byte = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        first_byte | second_byte << 8 // ssff
    }

    // Absolute X
    // 16-bit address is retrived from the next two bytes in the program counter offset by X
    pub fn mode_abx(&mut self) -> u16 {
        let first_byte = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        let second_byte = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        (first_byte | second_byte << 8) + self.x as u16 // ssff
    }

    // Absolute Y
    // 16-bit address is retrived from the next two bytes in the program counter offset by Y
    pub fn mode_aby(&mut self) -> u16 {
        let first_byte = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        let second_byte = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        (first_byte | second_byte << 8) + self.y as u16 // ssff
    }

    // Accumulator
    // Address is the accumulator, so no need to seek
    pub fn mode_acc(&mut self) -> u16 {
        0
    }

    // Immediate
    // Uses the address of the program counter for operation
    pub fn mode_imm(&mut self) -> u16 {
        let pc = self.pc;
        self.pc_increase();
        pc
    }

    // Implied
    // Address is implied, so no need to seek
    pub fn mode_imp(&mut self) -> u16 {
        0
    }

    // Indirect
    // Used for JMP, essentially the same concept as pointers
    pub fn mode_ind(&mut self) -> u16 {
        let first_byte = self.bus.get_memory(self.pc) as u16; // ff
        self.pc_increase();
        let second_byte = self.bus.get_memory(self.pc) as u16; // ss
        self.pc_increase();

        let address = first_byte | second_byte << 8; // ssff

        address 
    }

    // Indirect X-indexed
    // ???????????????
    pub fn mode_izx(&mut self) -> u16 {
        0
    }

    // Indirect Y-indexed
    // ???????????????
    pub fn mode_izy(&mut self) -> u16 {
        0
    }

    // Relative
    // Address is applied an offset from the PC
    pub fn mode_rel(&mut self) -> u16 {
        let pc = self.pc;
        self.pc_increase();
        return pc + (self.bus.get_memory(pc) - 128) as u16 // takes a signed byte
    }

    // Zero Page
    // Address is fetched from 8-bit address on the zero page
    pub fn mode_zp(&mut self) -> u16 {
        let pc = self.pc;
        let byte = self.bus.get_memory(pc) as u16;
        self.pc_increase();
        self.bus.get_memory(byte) as u16
    }

    // Zero Page X-indexed
    // Address is fetched from 8-bit address on the zero page, offset by X
    pub fn mode_zpx(&mut self) -> u16 {
        let pc = self.pc;
        let byte = self.bus.get_memory(pc) as u16;
        self.pc_increase();
        self.bus.get_memory((byte + self.x as u16) % 256) as u16
    }

    // Zero Page Y-indexed
    // Address is fetched from 8-bit address on the zero page, offset by Y
    pub fn mode_zpy(&mut self) -> u16 {
        let pc = self.pc;
        let byte = self.bus.get_memory(pc) as u16;
        self.pc_increase();
        self.bus.get_memory((byte + self.y as u16) % 256) as u16
    }
}

pub enum Mode {
    ABS,    // Absolute
    ABX,    // Absolute, X-indexed
    ABY,    // Absolute, Y-indexed
    ACC,    // Accumulator
    IMM,    // Immediate
    IMP,    // Implied
    IND,    // Indirect
    IZX,    // X-indexed, indirect
    IZY,    // Indirect, Y-indexed
    REL,    // Relative
    ZP,     // Zeropage
    ZPX,    // Zeropage, X-indexed
    ZPY     // Zeropage, Y-indexed
}