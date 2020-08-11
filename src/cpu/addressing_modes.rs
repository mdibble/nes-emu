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
        let left = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        let right = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        left << 4 | right
    }

    // Absolute X
    // 16-bit address is retrived from the next two bytes in the program counter offset by X
    pub fn mode_abx(&mut self) -> u16 {
        let left = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        let right = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        (left << 4 | right) + self.x as u16
    }

    // Absolute Y
    // 16-bit address is retrived from the next two bytes in the program counter offset by Y
    pub fn mode_aby(&mut self) -> u16 {
        let left = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        let right = self.bus.get_memory(self.pc) as u16;
        self.pc_increase();
        (left << 4 | right) + self.y as u16
    }

    // Accumulator
    // Address is the accumulator, so no need to seek
    pub fn mode_acc(&mut self) -> u16 {
        self.pc_increase();
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
        self.pc_increase();
        0
    }

    // Indirect
    // Need to learn more about this mode, pretty confusing
    pub fn mode_ind(&mut self) -> u16 {
        let address = self.bus.get_memory(self.pc + 1) as u16;
        let low = self.bus.get_memory(address) as u16;
        let high = if address & 0xFF == 0xFF {
            self.bus.get_memory(address as u16 - 0xFF) as u16
        } 
        else {
            self.bus.get_memory(address + 1) as u16
        };

        (high << 8) | low
    }

    // Indirect X-indexed
    // I thought indirect was bad
    pub fn mode_izx(&mut self) -> u16 {
        let address = self.bus.get_memory(self.pc + 1);
        let zp_low = address.wrapping_add(self.x);
        let zp_high = zp_low.wrapping_add(1);
        let zp_low_value = self.bus.get_memory(zp_low as u16) as u16;
        let zp_high_value = self.bus.get_memory(zp_high as u16) as u16;

        (zp_high_value << 8) | zp_low_value
    }

    // Indirect Y-indexed
    // I thought indirect was bad
    pub fn mode_izy(&mut self) -> u16 {
        let address = self.bus.get_memory(self.pc + 1);
        let zp_low = address;
        let zp_high = zp_low.wrapping_add(1);
        let zp_low_value = self.bus.get_memory(zp_low as u16) as u16;
        let zp_high_value = self.bus.get_memory(zp_high as u16) as u16;

        let old_address = (zp_high_value << 8) | zp_low_value;
        let new_address = old_address.wrapping_add(self.y as u16);

        new_address
    }

    // Relative
    // Address is applied an offset from the PC
    pub fn mode_rel(&mut self) -> u16 {
        let pc = self.pc;
        self.pc_increase();
        self.pc + self.bus.get_memory(pc) as u16 // May cause overflow issues
        // Should return a signed byte, which it does not.
    }

    // Zero Page
    // Address is fetched from 8-bit address on the zero page
    pub fn mode_zp(&mut self) -> u16 {
        let pc = self.pc;
        self.pc_increase();
        self.bus.get_memory(pc) as u16
    }

    // Zero Page X-indexed
    // Address is fetched from 8-bit address on the zero page, offset by X
    pub fn mode_zpx(&mut self) -> u16 {
        let pc = self.pc;
        self.pc_increase();
        self.bus.get_memory((pc + self.x as u16) % 256) as u16
    }

    // Zero Page Y-indexed
    // Address is fetched from 8-bit address on the zero page, offset by Y
    pub fn mode_zpy(&mut self) -> u16 {
        let pc = self.pc;
        self.pc_increase();
        self.bus.get_memory((pc + self.y as u16) % 256) as u16
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