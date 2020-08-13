use crate::cpu::CPU;
use super::addressing_modes::Mode;

impl CPU {
    pub fn adc(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn and(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.a & self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn asl(&mut self, mode: Mode) {
        let acc_mode = match mode {
            Mode::ACC => true,
            _ => false
        };

        let (address, _) = self.set_mode(mode);

        if address & 0b10000000 == 0b10000000 { self.set_carry(true); } else { self.set_carry(false); }
        let result = self.bus.get_memory(address) << 1;
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }

        if acc_mode == true {
            self.a = result;
            if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        }

        else {
            self.bus.write_memory(address, result);
        }
    }

    pub fn bcc(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn bcs(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn beq(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn bit(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
    }

    pub fn bmi(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn bne(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn bpl(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn brk(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
    }

    pub fn bvc(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn bvs(&mut self, mode: Mode) { // extra cycle if branching, two extra if on another page 
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn clc(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_carry(false);
    }

    pub fn cld(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_decimal(false);
    }

    pub fn cli(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_interrupt_disable(false);
    }

    pub fn clv(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_overflow(false);
    }

    pub fn cmp(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        if self.a >= self.bus.get_memory(address) { self.set_carry(true); } else { self.set_carry(false); }
        if self.a == self.bus.get_memory(address) { self.set_zero(true); } else { self.set_zero(false); }
        if self.a < self.bus.get_memory(address) { self.set_negative(true); } else { self.set_negative(false); } 
    }

    pub fn cpx(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        if self.x >= self.bus.get_memory(address) { self.set_carry(true); } else { self.set_carry(false); }
        if self.x == self.bus.get_memory(address) { self.set_zero(true); } else { self.set_zero(false); }
        if self.x < self.bus.get_memory(address) { self.set_negative(true); } else { self.set_negative(false); } 
    }

    pub fn cpy(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        if self.y >= self.bus.get_memory(address) { self.set_carry(true); } else { self.set_carry(false); }
        if self.y == self.bus.get_memory(address) { self.set_zero(true); } else { self.set_zero(false); }
        if self.y < self.bus.get_memory(address) { self.set_negative(true); } else { self.set_negative(false); } 
    }

    pub fn dec(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        let new_value = self.bus.write_memory(address, self.bus.get_memory(address) - 1);
        if new_value == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if new_value & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn dex(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.x -= 1;
        if self.x == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn dey(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.y -= 1;
        if self.y == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn eor(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.a ^ self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn inc(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        let new_value = self.bus.write_memory(address, self.bus.get_memory(address) + 1);
        if new_value == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if new_value & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn inx(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.x += 1;
        if self.x == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn iny(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.y += 1;
        if self.y == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn jmp(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        self.pc = address;
    }

    pub fn jsr(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        self.push(self.bus.get_memory(self.pc));
        self.pc = address;
    }

    pub fn lda(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn ldx(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        self.x = self.bus.get_memory(address);
        if self.x == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn ldy(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        self.y = self.bus.get_memory(address);
        if self.y == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn lsr(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
    }

    pub fn nop(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
    }

    pub fn ora(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.a | self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn pha(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.push(self.a);
    }

    pub fn php(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.push(self.p);
    }

    pub fn pla(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.a = self.sp;
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn plp(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.p = self.sp;
    }

    pub fn rol(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
    }

    pub fn ror(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
    }

    pub fn rti(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
    }

    pub fn rts(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.pc = self.bus.get_memory(self.sp as u16) as u16 - 1;
    }

    pub fn sbc(&mut self, mode: Mode) {
        let (address, extra_cycle) = self.set_mode(mode);
    }

    pub fn sec(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_carry(true);
    }

    pub fn sed(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_decimal(true);
    }

    pub fn sei(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.set_interrupt_disable(true);
    }

    pub fn sta(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        self.bus.write_memory(address, self.a);
    }

    pub fn stx(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        self.bus.write_memory(address, self.x);
    }

    pub fn sty(&mut self, mode: Mode) {
        let (address, _) = self.set_mode(mode);
        self.bus.write_memory(address, self.y);
    }

    pub fn tax(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.x = self.a;
        if self.x == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn tay(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.y = self.a;
        if self.y == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn tsx(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.x = self.bus.get_memory(self.sp as u16);
        if self.x == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn txa(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.a = self.x;
        if self.a == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }

    pub fn txs(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.sp = self.x;
    }

    pub fn tya(&mut self, mode: Mode) {
        let (_, _) = self.set_mode(mode);
        self.a = self.y;
        if self.a == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
    }
}