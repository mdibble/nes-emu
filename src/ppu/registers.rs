use crate::ppu::PPU;

impl PPU {
    pub fn read_ppu_status(&self) -> u8 {
        self.reg_ppu_status
    }
    pub fn read_oam_data(&self) -> u8 {
        self.reg_oam_data
    }

    pub fn read_ppu_data(&self) -> u8 {
        self.reg_ppu_data
    }

    pub fn write_ppu_ctrl(&mut self, contents: u8) -> u8 {
        self.reg_ppu_ctrl = contents;
        self.reg_ppu_ctrl
    }

    pub fn write_ppu_mask(&mut self, contents: u8) -> u8 {
        self.reg_ppu_mask = contents;
        self.reg_ppu_mask
    }

    pub fn write_oam_addr(&mut self, contents: u8) -> u8 {
        self.reg_oam_addr = contents;
        self.reg_oam_addr
    }

    pub fn write_oam_data(&mut self, contents: u8) -> u8 {
        self.reg_oam_data = contents;
        self.reg_oam_data
    }

    pub fn write_ppu_scroll(&mut self, contents: u8) -> u8 {
        self.reg_ppu_scroll = contents;
        self.reg_ppu_scroll
    }

    pub fn write_ppu_addr(&mut self, contents: u8) -> u8 {
        self.reg_ppu_addr = contents;
        self.reg_ppu_addr
    }

    pub fn write_ppu_data(&mut self, contents: u8) -> u8 {
        self.reg_ppu_mask = contents;
        self.reg_ppu_data
    }
}