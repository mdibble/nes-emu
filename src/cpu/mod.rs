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

    cycles: u8,
    opcode: u8,
    fetched: u8,
    addr_abs: u16,
    addr_rel: u16
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

            cycles: 0,
            opcode: 0x00,
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x00
        };
        cpu
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.bus.get_memory(self.pc);
            self.pc += 1;
            self.cycles = cycle_table[self.opcode as usize];
            
            if self.execute(self.opcode) {
                self.cycles += 1;
                // Should never happen as of now
            }
        }

        self.cycles -= 1;
    }

    pub fn reset() {

    }

    pub fn irq() {

    }

    pub fn nmi() {

    }

    pub fn execute(&mut self, opcode: u8) -> bool {
        let mut extend = false;
        match opcode {
            0x00 => extend = self.brk(Mode::IMP),
            0x01 => extend = self.ora(Mode::IZX),
            0x05 => extend = self.ora(Mode::ZP),
            0x06 => extend = self.asl(Mode::ZP),
            0x08 => extend = self.php(Mode::IMP),
            0x09 => extend = self.ora(Mode::IMM),
            0x0A => extend = self.asl(Mode::ACC),
            0x0D => extend = self.ora(Mode::ABS),
            0x0E => extend = self.asl(Mode::ABS),
            0x10 => extend = self.bpl(Mode::REL),
            0x11 => extend = self.ora(Mode::IZY),
            0x15 => extend = self.ora(Mode::ZPX),
            0x16 => extend = self.asl(Mode::ZPX),
            0x18 => extend = self.clc(Mode::IMP),
            0x19 => extend = self.ora(Mode::ABY),
            0x1D => extend = self.ora(Mode::ABX),
            0x1E => extend = self.asl(Mode::ABX),
            0x20 => extend = self.jsr(Mode::ABS),
            0x21 => extend = self.and(Mode::IZX),
            0x24 => extend = self.bit(Mode::ZP),
            0x25 => extend = self.and(Mode::ZP),
            0x26 => extend = self.rol(Mode::ZP),
            0x28 => extend = self.plp(Mode::IMP),
            0x29 => extend = self.and(Mode::IMM),
            0x2A => extend = self.rol(Mode::ACC),
            0x2C => extend = self.bit(Mode::ABS),
            0x2D => extend = self.and(Mode::ABS),
            0x2E => extend = self.rol(Mode::ABS),
            0x30 => extend = self.bmi(Mode::REL),
            0x31 => extend = self.and(Mode::IZY),
            0x35 => extend = self.and(Mode::ZPX),
            0x36 => extend = self.rol(Mode::ZPX),
            0x38 => extend = self.sec(Mode::IMP),
            0x39 => extend = self.and(Mode::ABY),
            0x3D => extend = self.and(Mode::ABX),
            0x3E => extend = self.rol(Mode::ABX),
            0x40 => extend = self.rti(Mode::IMP),
            0x41 => extend = self.eor(Mode::IZX),
            0x45 => extend = self.eor(Mode::ZP),
            0x46 => extend = self.lsr(Mode::ZP),
            0x48 => extend = self.pha(Mode::IMP),
            0x49 => extend = self.eor(Mode::IMM),
            0x4A => extend = self.lsr(Mode::ACC),
            0x4C => extend = self.jmp(Mode::ABS),
            0x4D => extend = self.eor(Mode::ABS),
            0x4E => extend = self.lsr(Mode::ABS),
            0x50 => extend = self.bvc(Mode::REL),
            0x51 => extend = self.eor(Mode::IZY),
            0x55 => extend = self.eor(Mode::ZPX),
            0x56 => extend = self.lsr(Mode::ZPX),
            0x58 => extend = self.cli(Mode::IMP),
            0x59 => extend = self.eor(Mode::ABY),
            0x5D => extend = self.eor(Mode::ABX),
            0x5E => extend = self.lsr(Mode::ABX),
            0x60 => extend = self.rts(Mode::IMP),
            0x61 => extend = self.adc(Mode::IZX),
            0x65 => extend = self.adc(Mode::ZP),
            0x66 => extend = self.ror(Mode::ZP),
            0x68 => extend = self.pla(Mode::IMP),
            0x69 => extend = self.adc(Mode::IMM),
            0x6A => extend = self.ror(Mode::ACC),
            0x6C => extend = self.jmp(Mode::IND),
            0x6D => extend = self.adc(Mode::ABS),
            0x6E => extend = self.ror(Mode::ABS),
            0x70 => extend = self.bvs(Mode::REL),
            0x71 => extend = self.adc(Mode::IZY),
            0x75 => extend = self.adc(Mode::ZPX),
            0x76 => extend = self.ror(Mode::ZPX),
            0x78 => extend = self.sei(Mode::IMP),
            0x79 => extend = self.adc(Mode::ABY),
            0x7D => extend = self.adc(Mode::ABX),
            0x7E => extend = self.ror(Mode::ABX),
            0x81 => extend = self.sta(Mode::IZX),
            0x84 => extend = self.sty(Mode::ZP),
            0x85 => extend = self.sta(Mode::ZP),
            0x86 => extend = self.stx(Mode::ZP),
            0x88 => extend = self.dey(Mode::IMP),
            0x8A => extend = self.txa(Mode::IMP),
            0x8C => extend = self.sty(Mode::ABS),
            0x8D => extend = self.sta(Mode::ABS),
            0x8E => extend = self.stx(Mode::ABS),
            0x90 => extend = self.bcc(Mode::REL),
            0x91 => extend = self.sta(Mode::IZY),
            0x94 => extend = self.sty(Mode::ZPX),
            0x95 => extend = self.sta(Mode::ZPX),
            0x96 => extend = self.stx(Mode::ZPY),
            0x98 => extend = self.tya(Mode::IMP),
            0x99 => extend = self.sta(Mode::ABY),
            0x9A => extend = self.txs(Mode::IMP),
            0x9D => extend = self.sta(Mode::ABX),
            0xA0 => extend = self.ldy(Mode::IMM),
            0xA1 => extend = self.lda(Mode::IZX),
            0xA2 => extend = self.ldx(Mode::IMM),
            0xA4 => extend = self.ldy(Mode::ZP),
            0xA5 => extend = self.lda(Mode::ZP),
            0xA6 => extend = self.ldx(Mode::ZP),
            0xA8 => extend = self.tay(Mode::IMP),
            0xA9 => extend = self.lda(Mode::ABY),
            0xAA => extend = self.tax(Mode::IMP),
            0xAC => extend = self.ldy(Mode::ABS),
            0xAD => extend = self.lda(Mode::ABS),
            0xAE => extend = self.ldx(Mode::ABS),
            0xB0 => extend = self.bcs(Mode::REL),
            0xB1 => extend = self.lda(Mode::IZY),
            0xB4 => extend = self.ldy(Mode::ZPX),
            0xB5 => extend = self.lda(Mode::ZPX),
            0xB6 => extend = self.ldx(Mode::ZPY),
            0xB8 => extend = self.clv(Mode::IMP),
            0xB9 => extend = self.lda(Mode::ABY),
            0xBA => extend = self.tsx(Mode::IMP),
            0xBC => extend = self.ldy(Mode::ABX),
            0xBD => extend = self.lda(Mode::ABX),
            0xBE => extend = self.ldx(Mode::ABY),
            0xC0 => extend = self.cpy(Mode::IMM),
            0xC1 => extend = self.cmp(Mode::IZX),
            0xC4 => extend = self.cpy(Mode::ZP),
            0xC5 => extend = self.cmp(Mode::ZP),
            0xC6 => extend = self.dec(Mode::ZP),
            0xC8 => extend = self.iny(Mode::IMP),
            0xC9 => extend = self.cmp(Mode::IMM),
            0xCA => extend = self.dex(Mode::IMP),
            0xCC => extend = self.cpy(Mode::ABS),
            0xCD => extend = self.cmp(Mode::ABS),
            0xCE => extend = self.dec(Mode::ABS),
            0xD0 => extend = self.bne(Mode::REL),
            0xD1 => extend = self.cmp(Mode::IZY),
            0xD5 => extend = self.cmp(Mode::ZPX),
            0xD6 => extend = self.dec(Mode::ZPX),
            0xD8 => extend = self.cld(Mode::IMP),
            0xD9 => extend = self.cmp(Mode::ABY),
            0xDD => extend = self.cmp(Mode::ABX),
            0xDE => extend = self.dec(Mode::ABX),
            0xE0 => extend = self.cpx(Mode::IMM),
            0xE1 => extend = self.sbc(Mode::IZX),
            0xE4 => extend = self.cpx(Mode::ZP),
            0xE5 => extend = self.sbc(Mode::ZP),
            0xE6 => extend = self.inc(Mode::ZP),
            0xE8 => extend = self.inx(Mode::IMP),
            0xE9 => extend = self.sbc(Mode::IMM),
            0xEA => extend = self.nop(Mode::IMP),
            0xEC => extend = self.cpx(Mode::ABS),
            0xED => extend = self.sbc(Mode::ABS),
            0xEE => extend = self.inc(Mode::ABS),
            0xF0 => extend = self.beq(Mode::REL),
            0xF1 => extend = self.sbc(Mode::IZY),
            0xF5 => extend = self.sbc(Mode::ZPX),
            0xF6 => extend = self.inc(Mode::ZPX),
            0xF8 => extend = self.sed(Mode::IMP),
            0xF9 => extend = self.sbc(Mode::ABY),
            0xFD => extend = self.sbc(Mode::ABX),
            0xFE => extend = self.inc(Mode::ABX),
            _ => panic!("[0x{:x}] No opcode! Could be illegal, unimplemented, or both.", opcode)
        }
        extend
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