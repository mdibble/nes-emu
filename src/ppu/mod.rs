pub struct PPU {
    memory: [u8; 0x4000], // $0000-$3FFF
    scanline: u16, // 260, y-axis
    cycle: u16, // 340, x-axis

    reg_ppu_ctrl: u8,       // $2000
    reg_ppu_mask: u8,       // $2001
    reg_ppu_status: u8,     // $2002
    reg_oam_addr: u8,       // $2003
    reg_oam_data: u8,       // $2004
    reg_ppu_scroll: u8,     // $2005
    reg_ppu_addr: u8,       // $2006
    reg_ppu_data: u8,       // $2007
}

impl PPU {
    pub fn new() -> PPU {
        let ppu = PPU {
            // bus reference
            memory: [0; 0x4000],
            scanline: 0,
            cycle: 0,
            reg_ppu_ctrl: 0b00000000,
            reg_ppu_mask: 0b00000000,
            reg_ppu_status: 0b00000000,
            reg_oam_addr: 0b00000000,
            reg_oam_data: 0b00000000,
            reg_ppu_scroll: 0b00000000,
            reg_ppu_addr: 0b00000000,
            reg_ppu_data: 0b00000000
        };
        ppu
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
        }

        // Drawing
        if self.scanline <= 239 {

        }

        // VBlank
        else if self.scanline == 241 && self.cycle == 1 {

        }

        //VBlank off
        else if self.scanline == 261 && self.cycle == 1 {

        }

    }

    pub fn get_reg(&self, address: u16) -> u8 {
        let result = match address {
            0x2000 => self.reg_ppu_ctrl,
            0x2001 => self.reg_ppu_mask,
            0x2002 => self.reg_ppu_status,
            0x2003 => self.reg_oam_addr,
            0x2004 => self.reg_oam_data,
            0x2005 => self.reg_ppu_scroll,
            0x2006 => self.reg_ppu_addr,
            0x2007 => self.reg_ppu_data,
            _ => panic!("No register at this location! ${:x}", address)
        };
        result
    }

    pub fn write_reg(&mut self, address: u16, contents: u8) -> u8 {
        match address {
            0x2000 => self.reg_ppu_ctrl = contents,
            0x2001 => self.reg_ppu_mask = contents,
            0x2002 => self.reg_ppu_status = contents,
            0x2003 => self.reg_oam_addr = contents,
            0x2004 => self.reg_oam_data = contents,
            0x2005 => self.reg_ppu_scroll = contents,
            0x2006 => self.reg_ppu_addr = contents,
            0x2007 => self.reg_ppu_data = contents,
            _ => panic!("No register at this location! ${:x}", address)
        }
        0
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) -> u8 {
        self.memory[address as usize] = contents;
        contents
    }
}