mod addressing_modes;
mod opcodes;
mod cycle_table;

use crate::bus::Bus;

use addressing_modes::Mode;
use cycle_table::cycle_table;

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
    pub bus: Bus,
}

impl CPU {
    pub fn new() -> CPU {
        let cpu = CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0, // Haven't verified
            sp: 0xFD,
            p: 0x34,
            bus: Bus::new(),
        };
        cpu
    }

    pub fn reset() {

    }

    pub fn irq() {

    }

    pub fn nmi() {

    }

    pub fn pc_increase(&mut self) {
        self.pc = self.pc.wrapping_add(0x1);
    }

    pub fn execute(&mut self, opcode: u8) {
        match opcode {
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
            0xA9 => self.lda(Mode::ABY),
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
    }

    pub fn set_carry(&mut self, state: bool) {
        self.p = if state { self.p | 0b00000001 } else { self.p & 0b11111110 };
    }

    pub fn set_zero(&mut self, state: bool) {
        self.p = if state { self.p | 0b00000010 } else { self.p & 0b11111101 };
    }

    pub fn set_interrupt(&mut self, state: bool) {
        self.p = if state { self.p | 0b00000100 } else { self.p & 0b11111011 };
    }

    pub fn set_decimal(&mut self, state: bool) {
        self.p = if state { self.p | 0b00001000 } else { self.p & 0b11110111 };
    }

    pub fn set_overflow(&mut self, state: bool) {
        self.p = if state { self.p | 0b01000000 } else { self.p & 0b10111111 };
    }

    pub fn set_negative(&mut self, state: bool) {
        self.p = if state { self.p | 0b10000000 } else { self.p & 0b01111111 };
    }
}