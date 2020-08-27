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
        // need to use the writing flag here
        self.reg_ppu_scroll
    }

    pub fn write_ppu_addr(&mut self, contents: u8) -> u8 {
        if self.writing == false {
            self.temp_address = (contents as u16) << 8;
            self.writing = true;
        }
        else {
            self.temp_address |= contents as u16;
            self.vram_address = self.temp_address;
            self.writing = false;
        }

        println!("{:04x}", self.temp_address);

        self.reg_ppu_addr = contents;
        //panic!("Tried to write to PPUADDR");
        self.reg_ppu_addr
    }

    pub fn write_ppu_data(&mut self, contents: u8) -> u8 {
        self.write_memory(self.vram_address, contents);
        if self.reg_ppu_ctrl & 0b00000100 == 0b00000100 {
            self.vram_address += 32;
        }
        else {
            self.vram_address += 1;
        }
        self.reg_ppu_data
    }
}