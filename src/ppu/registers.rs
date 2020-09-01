use crate::ppu::PPU;

impl PPU {
    pub fn read_ppu_status(&mut self) -> u8 {
        let val = self.reg_ppu_status;
        self.writing = false;
        self.reg_ppu_status &= 0b01111111; // clear vertical blank
        val
    }

    pub fn read_oam_data(&self) -> u8 {
        panic!("Tried to read OAM data");
        self.reg_oam_data
    }

    pub fn read_ppu_data(&self) -> u8 {
        panic!("Tried to read PPU data");
        let result = self.get_memory(self.vram_address);
        // Deal with buffering
        if self.reg_ppu_ctrl & 0b00000100 == 0b00000100 {
            self.vram_address += 32;
        }
        else {
            self.vram_address += 1;
        }
        result
    }

    pub fn write_ppu_ctrl(&mut self, contents: u8) -> u8 {
        self.reg_ppu_ctrl = contents;
        self.temp_address = (self.temp_address & 0xF3FF) | ((contents as u16 & 0x03) << 10);
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
        if self.writing == false {
            self.temp_address &= 0b0111111111100000;
            self.temp_address |= (contents as u16) >> 3;
            self.x_scroll = contents & 0b00000111;
            self.writing = true;
        }
        else {
            let contents = 0b10001101;
            self.temp_address &= 0b0000110000011111;
            self.temp_address |= (contents as u16 & 0x07) << 12;
            self.temp_address |= (contents as u16 & 0xF8) << 2;
            self.writing = false;
        }
        contents
    }

    pub fn write_ppu_addr(&mut self, contents: u8) -> u8 {
        if self.writing == false {
            self.temp_address &= 0b0000000011111111;
            self.temp_address |= (contents as u16 & 0b00111111) << 8;
            self.writing = true;
        }
        else {
            self.temp_address &= 0b0111111100000000;
            self.temp_address |= contents as u16;
            self.vram_address = self.temp_address;
            self.writing = false;
        }
        contents
    }

    pub fn write_ppu_data(&mut self, contents: u8) -> u8 {
        self.write_memory(self.vram_address, contents);
        if self.reg_ppu_ctrl & 0b00000100 == 0b00000100 {
            self.vram_address += 32;
        }
        else {
            self.vram_address += 1;
        }
        contents
    }
}