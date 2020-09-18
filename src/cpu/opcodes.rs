use crate::cpu::CPU;
use super::addressing_modes::Mode;

impl CPU {
    pub fn adc(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        
        let a = self.a;
        let b = self.bus.get_memory(address);
        let c = self.p & 1;
        self.a = a.wrapping_add(b).wrapping_add(c);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if a as u16 + b as u16 + c as u16 > 0xFF {
            self.p |= 0x1;
        } else {
            self.p &= !0x1;
        }
        if (a ^ b) & 0x80 == 0 && (a ^ self.a) & 0x80 != 0 {
            self.p |= 0b01000000;
        } else {
            self.p &= 0b10111111;
        }

        extra_cycle
    }

    pub fn and(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.a & self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn asl(&mut self, mode: Mode) -> u8 {
        let acc_mode = match mode {
            Mode::ACC => true,
            _ => false
        };

        let (address, _) = self.set_mode(mode);

        let operand = if acc_mode { self.a } else { self.bus.get_memory(address) };
        let result = operand << 1;
        if operand & 0b10000000 == 0b10000000 { self.set_carry(true); } else { self.set_carry(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if result == 0 { self.set_zero(true); } else { self.set_zero(false); }

        if acc_mode == true {
            self.a = result;
        }
        else {
            self.bus.write_memory(address, result);
        }
        0
    }

    pub fn bcc(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_carry() == false {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn bcs(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_carry() == true {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn beq(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_zero() == true {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn bit(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        let result = self.bus.get_memory(address);
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if (result >> 6) & 1 != 0 { self.set_overflow(true); } else { self.set_overflow(false); }
        if (result & self.a) == 0 { self.set_zero(true); } else { self.set_zero(false); }
        0
    }

    pub fn bmi(&mut self, mode: Mode) -> u8 { 
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_negative() == true {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn bne(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_zero() == false {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn bpl(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_negative() == false {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn brk(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.pc += 1;
        self.set_interrupt_disable(true);
        self.push((self.pc >> 8) as u8 & 0xFF);
        self.push(self.pc as u8 & 0xFF);
        self.push(self.p | 0x10);
        self.pc = self.bus.get_memory(0xFFFE) as u16 | (self.bus.get_memory(0xFFFF) as u16) << 8;
        0
    }

    pub fn bvc(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_overflow() == false {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn bvs(&mut self, mode: Mode) -> u8 {
        let (address, mut extra_cycle) = self.set_mode(mode);
        if self.get_overflow() == true {
            self.pc = address;
            extra_cycle += 1;
        }
        else {
            extra_cycle = 0;
        }
        extra_cycle
    }

    pub fn clc(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_carry(false);
        0
    }

    pub fn cld(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_decimal(false);
        0
    }

    pub fn cli(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_interrupt_disable(false);
        0
    }

    pub fn clv(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_overflow(false);
        0
    }

    pub fn cmp(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        let result: u8 = self.a.wrapping_sub(self.bus.get_memory(address));

        if self.a >= self.bus.get_memory(address) { self.set_carry(true); } else { self.set_carry(false); }
        if self.a == self.bus.get_memory(address) { self.set_zero(true); } else { self.set_zero(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn cpx(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        let result: u8 = self.x.wrapping_sub(self.bus.get_memory(address));

        if self.x >= self.bus.get_memory(address) { self.set_carry(true); } else { self.set_carry(false); }
        if self.x == self.bus.get_memory(address) { self.set_zero(true); } else { self.set_zero(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn cpy(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        let result: u8 = self.y.wrapping_sub(self.bus.get_memory(address));

        if self.y >= self.bus.get_memory(address) { self.set_carry(true); } else { self.set_carry(false); }
        if self.y == self.bus.get_memory(address) { self.set_zero(true); } else { self.set_zero(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn dec(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        let added = self.bus.get_memory(address).wrapping_sub(1);
        let new_value = self.bus.write_memory(address, added);
        if new_value == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if new_value & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn dex(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.x = self.x.wrapping_sub(1);
        if self.x == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn dey(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.y = self.y.wrapping_sub(1);
        if self.y == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn eor(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.a ^ self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn inc(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        let added = self.bus.get_memory(address).wrapping_add(1);
        let new_value = self.bus.write_memory(address, added);
        if new_value == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if new_value & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn inx(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        if self.x != 255 { self.x += 1; } else { self.x = 0; }
        if self.x == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn iny(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        
        if self.y != 255 { self.y += 1; } else { self.y = 0; }
        if self.y == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn jmp(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        self.pc = address;
        0
    }

    pub fn jsr(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        self.pc -= 1; // testing
        self.push((self.pc >> 8) as u8);
        self.push(self.pc as u8);
        self.pc = address;
        0
    }

    pub fn lda(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn ldx(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        self.x = self.bus.get_memory(address);
        if self.x == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn ldy(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        self.y = self.bus.get_memory(address);
        if self.y == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn lsr(&mut self, mode: Mode) -> u8 {
        let acc_mode = match mode {
            Mode::ACC => true,
            _ => false
        };

        let (address, _) = self.set_mode(mode);
        
        let operand = if acc_mode { self.a } else { self.bus.get_memory(address) };

        let result = operand >> 1;
        if operand & 0b00000001 == 0b00000001 { self.set_carry(true); } else { self.set_carry(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if result == 0 { self.set_zero(true); } else { self.set_zero(false); }

        if acc_mode == true {
            self.a = result;
        }
        else {
            self.bus.write_memory(address, result);
        }
        0
    }

    pub fn nop(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        0
    }

    pub fn ora(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        self.a = self.a | self.bus.get_memory(address);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        extra_cycle
    }

    pub fn pha(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.push(self.a);
        0
    }

    pub fn php(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.push(self.p | 0x10);
        0
    }

    pub fn pla(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.a = self.pop();
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn plp(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        let flags = self.pop();
        self.set_negative(flags & 0x80 != 0);
        self.set_overflow(flags & 0x40 != 0);
        self.set_decimal(flags & 0x08 != 0);
        self.set_interrupt_disable(flags & 0x04 != 0);
        self.set_zero(flags & 0x02 != 0);
        self.set_carry(flags & 0x01 != 0);
        0
    }

    pub fn rol(&mut self, mode: Mode) -> u8 {
        let acc_mode = match mode {
            Mode::ACC => true,
            _ => false
        };

        let (address, _) = self.set_mode(mode);
        let operand = if acc_mode { self.a } else { self.bus.get_memory(address) };
        let mut result = operand << 1;
        if self.get_carry() == true {
            result = result | 0b00000001;
        }
        if operand & 0b10000000 == 0b10000000 { self.set_carry(true); } else { self.set_carry(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if result == 0 { self.set_zero(true); } else { self.set_zero(false); }

        if acc_mode == true {
            self.a = result;
        }
        else {
            self.bus.write_memory(address, result);
        }
        0
    }

    pub fn ror(&mut self, mode: Mode) -> u8 {
        let acc_mode = match mode {
            Mode::ACC => true,
            _ => false
        };

        let (address, _) = self.set_mode(mode);
        let operand = if acc_mode { self.a } else { self.bus.get_memory(address) };
        let mut result = operand >> 1;
        if self.get_carry() == true {
            result = result | 0b10000000;
        }
        if operand & 0b00000001 == 0b00000001 { self.set_carry(true); } else { self.set_carry(false); }
        if result & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if result == 0 { self.set_zero(true); } else { self.set_zero(false); }

        if acc_mode == true {
            self.a = result;
        }
        else {
            self.bus.write_memory(address, result);
        }
        0
    }

    pub fn rti(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.p = self.pop() | 0x20;

        let lo = self.pop() as u16;
        let hi = self.pop() as u16;

        self.pc = (hi << 8) | lo;
        0
    }

    pub fn rts(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);

        let lo = self.pop() as u16;
        let hi = self.pop() as u16;

        self.pc = (hi << 8) | lo;
        self.pc += 1;
        0
    }

    pub fn sbc(&mut self, mode: Mode) -> u8 {
        let (address, extra_cycle) = self.set_mode(mode);
        
        let a = self.a;
        let b = self.bus.get_memory(address);
        let c = self.p & 1;
        self.a = a.wrapping_sub(b).wrapping_sub(1 - c);
        if self.a == 0 { self.set_zero(true); } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        if a as i16 - b as i16 - (1 - c) as i16 >= 0x0 {
            self.p |= 0x1;
        } else {
            self.p &= !0x1;
        }
        if (a ^ b) & 0x80 != 0 && (a ^ self.a) & 0x80 != 0 {
            self.p |= 0b01000000;
        } else {
            self.p &= 0b10111111;
        }

        extra_cycle
    }

    pub fn sec(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_carry(true);
        0
    }

    pub fn sed(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_decimal(true);
        0
    }

    pub fn sei(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.set_interrupt_disable(true);
        0
    }

    pub fn sta(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        self.bus.write_memory(address, self.a);
        0
    }

    pub fn stx(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        self.bus.write_memory(address, self.x);
        0
    }

    pub fn sty(&mut self, mode: Mode) -> u8 {
        let (address, _) = self.set_mode(mode);
        self.bus.write_memory(address, self.y);
        0
    }

    pub fn tax(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.x = self.a;
        if self.x == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn tay(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.y = self.a;
        if self.y == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.y & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn tsx(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.x = self.sp;
        if self.x == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.x & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn txa(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.a = self.x;
        if self.a == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }

    pub fn txs(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.sp = self.x;
        0
    }

    pub fn tya(&mut self, mode: Mode) -> u8 {
        let (_, _) = self.set_mode(mode);
        self.a = self.y;
        if self.a == 0 { self.set_zero(true) } else { self.set_zero(false); }
        if self.a & 0b10000000 == 0b10000000 { self.set_negative(true); } else { self.set_negative(false); }
        0
    }
}