mod addressing_modes;
mod opcodes;
mod cycle_table;

use crate::bus::Bus;

use addressing_modes::Mode;
use cycle_table::CYCLE_TABLE;

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
    pub bus: Bus,
    cycles: u8,
    stall: u16,
    total_cycles: usize
}

impl CPU {
    pub fn new(cart_data: Vec<u8>) -> CPU {
        let cpu = CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFD,
            p: 0x24,
            bus: Bus::new(cart_data),
            cycles: 0,
            stall: 0,
            total_cycles: 0
        };
        cpu
    }

    pub fn tick(&mut self) {
        if self.bus.dma_page != 0 {
            let page = self.bus.dma_page;
            self.bus.dma_page = 0;

            for i in 0..256 as u16 {
                self.bus.ppu.oam_memory[i as usize] = self.bus.get_memory((256 * (page as u16)) + i);
            }

            self.stall += 513;
            if self.total_cycles % 2 == 1 {
                self.stall += 1;
            }
            return
        }

        if self.stall > 0 {
            self.stall -= 1;
            return
        }

        if self.cycles == 0 {
            if self.bus.ppu.trigger_nmi {
                self.bus.ppu.trigger_nmi = false;
                self.nmi();
            }
            else {
                let opcode = self.bus.get_memory(self.pc);

                // print!("${:04x}:\t0x{:02x}\t({:02x} {:02x})\t\t", self.pc, opcode, self.bus.get_memory(self.pc + 1), self.bus.get_memory(self.pc + 2));
                // print!("A:{:02x}\tX:{:02x}\tY:{:02x}\tP:{:02x}\tSP:{:02x}\tPPU:{}, {}\tCYC:{}\n", self.a, self.x, self.y, self.p, self.sp, self.bus.ppu.cycle, self.bus.ppu.scanline, self.total_cycles);
                
                self.pc_increase();
                self.cycles = self.execute(opcode) + CYCLE_TABLE[opcode as usize] as u8;
                self.cycles -= 1;
                self.total_cycles += 1;
            }
        }
        
        else {
            self.cycles -= 1;
            self.total_cycles += 1;
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.p = 0x24;

        self.total_cycles += 7;
        self.cycles = 0;

        // NMI: $FFFA-$FFFB
        // RESET: $FFFC-$FFFD
        // IRQ: $FFFE-$FFFF

        self.bus.reset();

        let first = self.bus.get_memory(0xFFFD) as u16;
        let second = self.bus.get_memory(0xFFFC) as u16;
        self.pc = first << 8 | second;
    }

    pub fn irq(&mut self) {
        if self.get_interrupt_disable() == false {
            self.push((self.pc >> 8) as u8 & 0xFF);
            self.push(self.pc as u8 & 0xFF);
            self.push(self.p);
            self.set_interrupt_disable(true);
            self.set_b_01(true);

            let lo = self.bus.get_memory(0xFFFE) as u16;
            let hi = self.bus.get_memory(0xFFFF) as u16;

            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }

    pub fn nmi(&mut self) {
        self.push((self.pc >> 8) as u8 & 0xFF);
        self.push(self.pc as u8 & 0xFF);
        self.push(self.p | 0x10);
        self.set_interrupt_disable(true);
        self.set_b_01(true);

        let lo = self.bus.get_memory(0xFFFA) as u16;
        let hi = self.bus.get_memory(0xFFFB) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 7; 
    }

    pub fn pc_increase(&mut self) {
        self.pc = self.pc.wrapping_add(0x1);
    }

    pub fn push(&mut self, data: u8) {
        self.bus.write_memory(0x100 + self.sp as u16, data);
        self.sp = if self.sp == 0x00 { 0xFF } else { self.sp - 1 };
    }

    pub fn pop(&mut self) -> u8 {
        self.sp = if self.sp == 0xFF { 0x00 } else { self.sp + 1 };
        self.bus.get_memory(0x100 + self.sp as u16)
    }

    pub fn execute(&mut self, opcode: u8) -> u8 {
        let cycle_count = match opcode {
            0x00 => self.brk(Mode::IMP),
            0x01 => self.ora(Mode::IZX),
            0x05 => self.ora(Mode::ZP),
            0x06 => self.asl(Mode::ZP),
            0x08 => self.php(Mode::IMP),
            0x09 => self.ora(Mode::IMM),
            0x0A => self.asl(Mode::ACC),
            0x0D => self.ora(Mode::ABS),
            0x0E => self.asl(Mode::ABS),
            0x10 => self.bpl(Mode::REL),
            0x11 => self.ora(Mode::IZY),
            0x15 => self.ora(Mode::ZPX),
            0x16 => self.asl(Mode::ZPX),
            0x18 => self.clc(Mode::IMP),
            0x19 => self.ora(Mode::ABY),
            0x1D => self.ora(Mode::ABX),
            0x1E => self.asl(Mode::ABX),
            0x20 => self.jsr(Mode::ABS),
            0x21 => self.and(Mode::IZX),
            0x24 => self.bit(Mode::ZP),
            0x25 => self.and(Mode::ZP),
            0x26 => self.rol(Mode::ZP),
            0x28 => self.plp(Mode::IMP),
            0x29 => self.and(Mode::IMM),
            0x2A => self.rol(Mode::ACC),
            0x2C => self.bit(Mode::ABS),
            0x2D => self.and(Mode::ABS),
            0x2E => self.rol(Mode::ABS),
            0x30 => self.bmi(Mode::REL),
            0x31 => self.and(Mode::IZY),
            0x35 => self.and(Mode::ZPX),
            0x36 => self.rol(Mode::ZPX),
            0x38 => self.sec(Mode::IMP),
            0x39 => self.and(Mode::ABY),
            0x3D => self.and(Mode::ABX),
            0x3E => self.rol(Mode::ABX),
            0x40 => self.rti(Mode::IMP),
            0x41 => self.eor(Mode::IZX),
            0x45 => self.eor(Mode::ZP),
            0x46 => self.lsr(Mode::ZP),
            0x48 => self.pha(Mode::IMP),
            0x49 => self.eor(Mode::IMM),
            0x4A => self.lsr(Mode::ACC),
            0x4C => self.jmp(Mode::ABS),
            0x4D => self.eor(Mode::ABS),
            0x4E => self.lsr(Mode::ABS),
            0x50 => self.bvc(Mode::REL),
            0x51 => self.eor(Mode::IZY),
            0x55 => self.eor(Mode::ZPX),
            0x56 => self.lsr(Mode::ZPX),
            0x58 => self.cli(Mode::IMP),
            0x59 => self.eor(Mode::ABY),
            0x5D => self.eor(Mode::ABX),
            0x5E => self.lsr(Mode::ABX),
            0x60 => self.rts(Mode::IMP),
            0x61 => self.adc(Mode::IZX),
            0x65 => self.adc(Mode::ZP),
            0x66 => self.ror(Mode::ZP),
            0x68 => self.pla(Mode::IMP),
            0x69 => self.adc(Mode::IMM),
            0x6A => self.ror(Mode::ACC),
            0x6C => self.jmp(Mode::IND),
            0x6D => self.adc(Mode::ABS),
            0x6E => self.ror(Mode::ABS),
            0x70 => self.bvs(Mode::REL),
            0x71 => self.adc(Mode::IZY),
            0x75 => self.adc(Mode::ZPX),
            0x76 => self.ror(Mode::ZPX),
            0x78 => self.sei(Mode::IMP),
            0x79 => self.adc(Mode::ABY),
            0x7D => self.adc(Mode::ABX),
            0x7E => self.ror(Mode::ABX),
            0x81 => self.sta(Mode::IZX),
            0x84 => self.sty(Mode::ZP),
            0x85 => self.sta(Mode::ZP),
            0x86 => self.stx(Mode::ZP),
            0x88 => self.dey(Mode::IMP),
            0x8A => self.txa(Mode::IMP),
            0x8C => self.sty(Mode::ABS),
            0x8D => self.sta(Mode::ABS),
            0x8E => self.stx(Mode::ABS),
            0x90 => self.bcc(Mode::REL),
            0x91 => self.sta(Mode::IZY),
            0x94 => self.sty(Mode::ZPX),
            0x95 => self.sta(Mode::ZPX),
            0x96 => self.stx(Mode::ZPY),
            0x98 => self.tya(Mode::IMP),
            0x99 => self.sta(Mode::ABY),
            0x9A => self.txs(Mode::IMP),
            0x9D => self.sta(Mode::ABX),
            0xA0 => self.ldy(Mode::IMM),
            0xA1 => self.lda(Mode::IZX),
            0xA2 => self.ldx(Mode::IMM),
            0xA4 => self.ldy(Mode::ZP),
            0xA5 => self.lda(Mode::ZP),
            0xA6 => self.ldx(Mode::ZP),
            0xA8 => self.tay(Mode::IMP),
            0xA9 => self.lda(Mode::IMM),
            0xAA => self.tax(Mode::IMP),
            0xAC => self.ldy(Mode::ABS),
            0xAD => self.lda(Mode::ABS),
            0xAE => self.ldx(Mode::ABS),
            0xB0 => self.bcs(Mode::REL),
            0xB1 => self.lda(Mode::IZY),
            0xB4 => self.ldy(Mode::ZPX),
            0xB5 => self.lda(Mode::ZPX),
            0xB6 => self.ldx(Mode::ZPY),
            0xB8 => self.clv(Mode::IMP),
            0xB9 => self.lda(Mode::ABY),
            0xBA => self.tsx(Mode::IMP),
            0xBC => self.ldy(Mode::ABX),
            0xBD => self.lda(Mode::ABX),
            0xBE => self.ldx(Mode::ABY),
            0xC0 => self.cpy(Mode::IMM),
            0xC1 => self.cmp(Mode::IZX),
            0xC4 => self.cpy(Mode::ZP),
            0xC5 => self.cmp(Mode::ZP),
            0xC6 => self.dec(Mode::ZP),
            0xC8 => self.iny(Mode::IMP),
            0xC9 => self.cmp(Mode::IMM),
            0xCA => self.dex(Mode::IMP),
            0xCC => self.cpy(Mode::ABS),
            0xCD => self.cmp(Mode::ABS),
            0xCE => self.dec(Mode::ABS),
            0xD0 => self.bne(Mode::REL),
            0xD1 => self.cmp(Mode::IZY),
            0xD5 => self.cmp(Mode::ZPX),
            0xD6 => self.dec(Mode::ZPX),
            0xD8 => self.cld(Mode::IMP),
            0xD9 => self.cmp(Mode::ABY),
            0xDD => self.cmp(Mode::ABX),
            0xDE => self.dec(Mode::ABX),
            0xE0 => self.cpx(Mode::IMM),
            0xE1 => self.sbc(Mode::IZX),
            0xE4 => self.cpx(Mode::ZP),
            0xE5 => self.sbc(Mode::ZP),
            0xE6 => self.inc(Mode::ZP),
            0xE8 => self.inx(Mode::IMP),
            0xE9 => self.sbc(Mode::IMM),
            0xEA => self.nop(Mode::IMP),
            0xEC => self.cpx(Mode::ABS),
            0xED => self.sbc(Mode::ABS),
            0xEE => self.inc(Mode::ABS),
            0xF0 => self.beq(Mode::REL),
            0xF1 => self.sbc(Mode::IZY),
            0xF5 => self.sbc(Mode::ZPX),
            0xF6 => self.inc(Mode::ZPX),
            0xF8 => self.sed(Mode::IMP),
            0xF9 => self.sbc(Mode::ABY),
            0xFD => self.sbc(Mode::ABX),
            0xFE => self.inc(Mode::ABX),
            _ => panic!("[0x{:x}] No opcode! Could be illegal, unimplemented, or both.", opcode)
        };
        cycle_count
    }

    pub fn set_carry(&mut self, state: bool) { self.p = if state { self.p | 0b00000001 } else { self.p & 0b11111110 }; }
    pub fn get_carry(&self) -> bool { let val = if self.p & 0b00000001 == 0b00000001 { true } else { false }; val }

    pub fn set_zero(&mut self, state: bool) { self.p = if state { self.p | 0b00000010 } else { self.p & 0b11111101 }; }
    pub fn get_zero(&self) -> bool { let val = if self.p & 0b00000010 == 0b00000010 { true } else { false }; val }

    pub fn set_interrupt_disable(&mut self, state: bool) { self.p = if state { self.p | 0b00000100 } else { self.p & 0b11111011 }; }
    pub fn get_interrupt_disable(&self) -> bool { let val = if self.p & 0b00000100 == 0b00000100 { true } else { false }; val }

    pub fn set_decimal(&mut self, state: bool) { self.p = if state { self.p | 0b00001000 } else { self.p & 0b11110111 }; }
    pub fn get_decimal(&self) -> bool { let val = if self.p & 0b00001000 == 0b00001000 { true } else { false }; val }

    pub fn set_b_01(&mut self, state: bool) { self.p = if state { self.p | 0b00010000 } else { self.p & 0b11101111 }; }
    pub fn get_b_01(&self) -> bool { let val = if self.p & 0b00010000 == 0b00010000 { true } else { false }; val }

    pub fn set_b_10(&mut self, state: bool) { self.p = if state { self.p | 0b00100000 } else { self.p & 0b11011111 }; }
    pub fn get_b_10(&self) -> bool { let val = if self.p & 0b00100000 == 0b00100000 { true } else { false }; val }

    pub fn set_overflow(&mut self, state: bool) { self.p = if state { self.p | 0b01000000 } else { self.p & 0b10111111 }; } 
    pub fn get_overflow(&self) -> bool { let val = if self.p & 0b01000000 == 0b01000000 { true } else { false }; val }

    pub fn set_negative(&mut self, state: bool) { self.p = if state { self.p | 0b10000000 } else { self.p & 0b01111111 }; }
    pub fn get_negative(&self) -> bool { let val = if self.p & 0b10000000 == 0b10000000 { true } else { false }; val }
}