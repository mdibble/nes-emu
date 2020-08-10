use crate::cpu::CPU;
use super::addressing_modes::Mode;

impl CPU {
    pub fn adc(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn and(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn asl(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bcc(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bcs(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn beq(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bit(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bmi(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bne(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bpl(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn brk(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bvc(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn bvs(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn clc(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn cld(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn cli(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn clv(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn cmp(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn cpx(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn cpy(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn dec(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn dex(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn dey(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn eor(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn inc(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn inx(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn iny(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn jmp(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn jsr(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn lda(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn ldx(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn ldy(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn lsr(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn nop(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn ora(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn pha(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn rol(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn ror(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn rti(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn rts(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn php(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn pla(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn plp(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn sbc(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn sec(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn sed(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn sei(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn sta(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn stx(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn sty(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn tax(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn tay(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn tsx(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn txa(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn txs(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }

    pub fn tya(&mut self, mode: Mode) -> bool {
        self.set_mode(mode) & true
    }
}