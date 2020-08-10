use crate::cpu::CPU;

impl CPU {
    pub fn set_mode(&mut self, mode: Mode) -> bool {
        let mut extend = false;
        match mode {
            Mode::IMM => extend = self.mode_imm(), 
            Mode::ZP => extend = self.mode_zp(), 
            Mode::ZPX => extend = self.mode_zpx(), 
            Mode::ZPY => extend = self.mode_zpy(), 
            Mode::IZX => extend = self.mode_izx(), 
            Mode::IZY => extend = self.mode_izy(), 
            Mode::ABS => extend = self.mode_abs(), 
            Mode::ABX => extend = self.mode_abx(), 
            Mode::ABY => extend = self.mode_aby(),
            Mode::ACC => extend = self.mode_acc(),
            Mode::IND => extend = self.mode_ind(), 
            Mode::REL => extend = self.mode_rel(), 
            Mode::IMP => extend = self.mode_imp()
        }
        extend
    }

    pub fn mode_abs(&mut self) -> bool {
        let lo = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;
        let hi = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        false
    }

    pub fn mode_abx(&mut self) -> bool {
        let lo = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;
        let hi = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            true
        }
        else {
            false
        }
    }

    pub fn mode_aby(&mut self) -> bool {
        let lo = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;
        let hi = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            true
        }
        else {
            false
        }
    }

    // Don't know if this will work
    pub fn mode_acc(&mut self) -> bool {
        self.addr_abs = self.pc;
        self.pc += 1;
        false
    }

    pub fn mode_imm(&mut self) -> bool {
        self.addr_abs = self.pc;
        self.pc += 1;
        false
    }

    pub fn mode_imp(&mut self) -> bool {
        self.fetched = self.a;
        false
    }

    pub fn mode_ind(&mut self) -> bool {
        let ptr_lo = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;
        let ptr_hi = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            self.addr_abs = (self.bus.get_memory(ptr & 0xFF00) as u16) << 8 | self.bus.get_memory(ptr + 0) as u16;
        }

        else {
            self.addr_abs = (self.bus.get_memory(ptr + 1) as u16) << 8 | self.bus.get_memory(ptr + 0) as u16;
        }
        
        false
    }

    pub fn mode_izx(&mut self) -> bool {
        let t = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.bus.get_memory((t + self.x as u16) & 0x00FF).into();
        let hi: u16 = self.bus.get_memory((t + (self.x + 1) as u16) & 0x00FF).into();

        self.addr_abs = (hi << 8) | lo;

        false
    }

    pub fn mode_izy(&mut self) -> bool {
        let t = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.bus.get_memory(t & 0x00FF).into();
        let hi: u16 = self.bus.get_memory((t + 1) & 0x00FF).into();

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            true
        }
        else {
            false
        }
    }

    pub fn mode_rel(&mut self) -> bool {
        self.addr_rel = self.bus.get_memory(self.pc) as u16;
        self.pc += 1;

        if self.addr_rel & 0x80 != 0 {
            self.addr_rel |= 0xFF00
        }
        
        false
    }

    pub fn mode_zp(&mut self) -> bool {
        self.addr_abs = self.bus.get_memory(self.pc).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    pub fn mode_zpx(&mut self) -> bool {
        self.addr_abs = self.bus.get_memory(self.pc + self.x as u16).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    pub fn mode_zpy(&mut self) -> bool {
        self.addr_abs = self.bus.get_memory(self.pc + self.y as u16).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
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