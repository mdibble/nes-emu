mod registers;
mod colors;

use colors::SYS_COLORS;

use crate::cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    nametables: [u8; 0x800],
    palettes: [u8; 0x20],
    oam_memory: [u8; 0x100],
    pub scanline: u16, // 260, y-axis
    pub cycle: u16, // 340, x-axis

    reg_ppu_ctrl: u8,       // $2000
    reg_ppu_mask: u8,       // $2001
    reg_ppu_status: u8,     // $2002
    reg_oam_addr: u8,       // $2003
    reg_oam_data: u8,       // $2004

    x_scroll: u8,

    vram_address: u16,
    temp_address: u16,
    writing: bool,

    fetched_nt: u8,
    fetched_at: u8,
    fetched_bg_lo: u8,
    fetched_bg_hi: u8,

    shift_reg_pt_lo: u16,
    shift_reg_pt_hi: u16,

    shift_reg_palette_lo: u8,
    shift_reg_palette_hi: u8,
    palette_latch: u8,

    pub trigger_nmi: bool,

    cartridge: Option<Rc<RefCell<Cartridge>>>,

    vblank: bool,
    even_frame: bool,
    pub display: Vec<u8>,
    pub draw: bool
}

impl PPU {
    pub fn new() -> PPU {
        let ppu = PPU {
            nametables: [0; 0x800],
            palettes: [0; 0x20],
            oam_memory: [0; 0x100],
            scanline: 0,
            cycle: 0, // maybe should be 341?
            reg_ppu_ctrl: 0b00000000,
            reg_ppu_mask: 0b00000000,
            reg_ppu_status: 0b00000000,
            reg_oam_addr: 0b00000000,
            reg_oam_data: 0b00000000,

            x_scroll: 0,

            vram_address: 0x0000,
            temp_address: 0x0000,
            writing: false,

            fetched_nt: 0,
            fetched_at: 0,
            fetched_bg_lo: 0,
            fetched_bg_hi: 0,

            shift_reg_pt_lo: 0,
            shift_reg_pt_hi: 0,

            shift_reg_palette_lo: 0,
            shift_reg_palette_hi: 0,
            palette_latch: 0,

            trigger_nmi: false,

            cartridge: None,

            vblank: true,
            even_frame: true,
            display: vec![0x00; 256 * 240 * 3],
            draw: false
        };
        ppu
    }

    pub fn reset(&mut self) {
        self.cycle = 0;
        self.scanline = 0;
        self.reg_ppu_ctrl = 0b00000000;
        self.reg_ppu_mask = 0b00000000;
        self.reg_ppu_status = 0b00000000;
        self.reg_oam_addr = 0b00000000;
        self.reg_oam_data = 0b00000000;
    }

    pub fn assign_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
    }

    pub fn fetch_nt(&mut self) {
        let address = 0x2000 | (self.vram_address & 0x0FFF);
        self.fetched_nt = self.get_memory(address);
    }

    pub fn fetch_at(&mut self) {
        let address = 0x23C0 | (self.vram_address & 0x0C00) | ((self.vram_address >> 4) & 0x38) | ((self.vram_address >> 2) & 0x07);
        self.fetched_at = self.get_memory(address);
    }

    pub fn fetch_tile_lo(&mut self) {
        let background_table = (self.reg_ppu_ctrl >> 4) & 0x01;
        let mut address = 0x1000 * background_table as u16;
        address += self.fetched_nt as u16 * 16;
        address += (self.vram_address >> 12) & 7;
        self.fetched_bg_lo = self.get_memory(address);
    }

    pub fn fetch_tile_hi(&mut self) {
        let background_table = (self.reg_ppu_ctrl >> 4) & 0x01;
        let mut address = 0x1000 * background_table as u16;
        address += self.fetched_nt as u16 * 16;
        address += (self.vram_address >> 12) & 7;
        self.fetched_bg_hi = self.get_memory(address + 8);
    }

    pub fn store_tile_data(&mut self) {
        self.shift_reg_pt_lo = (self.shift_reg_pt_lo & 0xFF00) | (self.fetched_bg_lo as u16);
        self.shift_reg_pt_hi = (self.shift_reg_pt_hi & 0xFF00) | (self.fetched_bg_hi as u16);
        self.palette_latch = self.fetched_at;
    }

    pub fn update_tile_data(&mut self) {
        self.shift_reg_pt_lo <<= 1;
        self.shift_reg_pt_hi <<= 1;

        self.shift_reg_palette_lo <<= 1;
        self.shift_reg_palette_hi <<= 1;

        let latch_bit0 = self.palette_latch & 0b01;
        let latch_bit1 = (self.palette_latch & 0b10) >> 1;
        self.shift_reg_palette_lo |= latch_bit0;
        self.shift_reg_palette_hi |= latch_bit1;
    }

    pub fn render_pixel(&mut self) {
        let row = self.scanline;
        let col = self.cycle - 1;

        let pixel = (self.shift_reg_pt_hi >> 15) << 1 | self.shift_reg_pt_lo >> 15;
        let palette = (self.shift_reg_palette_hi >> 7) << 1 | self.shift_reg_palette_lo >> 7;

        let new_palette = self.get_memory(self.get_palette_address(palette, pixel));

        self.display[(row as usize * 256 * 3) + (col * 3) as usize + 0] = SYS_COLORS[new_palette as usize].r;
        self.display[(row as usize * 256 * 3) + (col * 3) as usize + 1] = SYS_COLORS[new_palette as usize].g;
        self.display[(row as usize * 256 * 3) + (col * 3) as usize + 2] = SYS_COLORS[new_palette as usize].b;
    }

    pub fn get_palette_address(&self, palette: u8, pixel: u16) -> u16 {
        if pixel == 0 {
            0x3F00
        }
        else {
            0x3F00 + (palette as u16 * 4) + pixel
        }
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;

                if self.even_frame {
                    self.cycle += 1;
                }
                self.even_frame = !self.even_frame;
            }
        }

        // Information of current scanline/cycle
        let enable_rendering = self.reg_ppu_mask & 0b00011000 != 0;
        let visible_scanline = self.scanline <= 239;
        let prerender_scanline = self.scanline == 261;
        let visible_cycle = self.cycle >= 1 && self.cycle <= 256;
        let fetch_cycle = self.cycle >= 321 && self.cycle <= 336;

        // Start of drawing

        if enable_rendering {
            if visible_scanline && visible_cycle {
                self.render_pixel();
            }

            // Handling VRAM fetches (beige blocks on NTSC timing diagram)
            if (visible_scanline || prerender_scanline) && (visible_cycle || fetch_cycle) {
                self.update_tile_data();
                match self.cycle % 8 {
                    1 => self.fetch_nt(),
                    3 => self.fetch_at(),
                    5 => self.fetch_tile_lo(),
                    7 => self.fetch_tile_hi(),
                    0 => self.store_tile_data(),
                    _ => {}
                }
            }

            if (visible_scanline || prerender_scanline) && (self.cycle == 337 || self.cycle == 339) {
                self.fetch_nt();
            }

            // Handling register changes (red blocks on NTSC timing diagram)
            if prerender_scanline && self.cycle >= 280 && self.cycle <= 304 {
                // vert(v) = vert(t)
                self.y_copy();
            }  

            if visible_scanline || prerender_scanline {
                if self.cycle % 8 == 0 && (visible_cycle || fetch_cycle) {
                    self.x_increment();
                }
                if self.cycle == 256 {
                    self.y_increment();
                }
                if self.cycle == 257 {
                    // hori(v) = hori(t)
                    self.x_copy();
                }
            }
        }

        // End of drawing

        // VBlank
        if self.scanline == 241 && self.cycle == 1 {
            self.draw = true; // ready to draw to canvas
            self.vblank = true;
            self.reg_ppu_status |= 0b10000000;
            if self.reg_ppu_ctrl & 0b10000000 == 0b10000000 {
                self.trigger_nmi = true;
            }
        }

        // VBlank off
        if self.scanline == 261 && self.cycle == 1 {
            self.vblank = false;
            self.reg_ppu_status &= 0b01111111;  
            self.reg_ppu_status &= 0b10111111;  // sprite 0 hit
            self.reg_ppu_status &= 0b11011111;  // sprite overflow
        }
    }

    pub fn x_increment(&mut self) {
        if self.vram_address & 0x001F == 31 {
            self.vram_address &= !0x001F;
            self.vram_address ^= 0x0400;
        }
        else {
            self.vram_address += 1;
        }
    }

    pub fn y_increment(&mut self) {
        if (self.vram_address & 0x7000) != 0x7000 {
            self.vram_address += 0x1000;
        }
        else {
            self.vram_address &= !0x7000;
            let mut y = (self.vram_address & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.vram_address ^= 0x0800;
            }
            else if y == 31 {
                y = 0;
            }
            else {
                y += 1;
            }
            self.vram_address = (self.vram_address & !0x03E0) | (y << 5);
        }
    }

    pub fn x_copy(&mut self) {
        self.vram_address &= !0b0000010000011111;
        self.vram_address |= self.temp_address & 0b0000010000011111;
    }

    pub fn y_copy(&mut self) {
        self.vram_address &= !0b0111101111100000;
        self.vram_address |= self.temp_address & !0b0111101111100000;
    }

    pub fn get_reg(&mut self, address: u16) -> u8 {
        let result = match address {
            0x2002 => self.read_ppu_status(),
            0x2004 => self.read_oam_data(),
            0x2007 => self.read_ppu_data(),
            _ => panic!("No register at this location! ${:x}", address)
        };
        result
    }

    pub fn write_reg(&mut self, address: u16, contents: u8) -> u8 {
        self.reg_ppu_status &= 0b11100000;
        self.reg_ppu_status = self.reg_ppu_status | (contents & 0b00011111);

        let result = match address {
            0x2000 => self.write_ppu_ctrl(contents),
            0x2001 => self.write_ppu_mask(contents),
            0x2003 => self.write_oam_addr(contents),
            0x2004 => self.write_oam_data(contents),
            0x2005 => self.write_ppu_scroll(contents),
            0x2006 => self.write_ppu_addr(contents),
            0x2007 => self.write_ppu_data(contents),
            _ => panic!("No register at this location! ${:x}", address)
        };
        result
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        let result = match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref cart) => cart.borrow().chr_read(address),
                _ => panic!("PPU unable to read address ${:x} from cartridge", address)
            }
            0x2000..=0x3EFF => {
                let mirror_mode = match self.cartridge {
                    Some(ref cart) => cart.borrow().mirroring,
                    _ => panic!("Unable to get cartridge mirror mode")
                };
                match mirror_mode {
                    0 => { // Horizontal
                        match address {
                            0x2000..=0x23FF => self.nametables[(address as usize - 0x2000)],
                            0x2400..=0x27FF => self.nametables[(address as usize - 0x2400)],
                            0x2800..=0x2BFF => self.nametables[(address as usize - 0x2800)],
                            0x2C00..=0x2FFF => self.nametables[(address as usize - 0x2C00)],
                            0x3000..=0x33FF => self.nametables[(address as usize - 0x3000)],
                            0x3400..=0x37FF => self.nametables[(address as usize - 0x3400)],
                            0x3800..=0x3BFF => self.nametables[(address as usize - 0x3800)],
                            0x3C00..=0x3EFF => self.nametables[(address as usize - 0x3C00)],
                            _ => panic!("Impossible address")
                        }
                    },
                    _ => panic!("Haven't dealt with mirroring")
                }
            },
            0x3F00..=0x3FFF => self.palettes[address as usize - 0x3F00],
            _ => panic!("PPU requested read outside of memory range: ${:x}", address)
        };
        result
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) -> u8 {
        let result = match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref cart) => cart.borrow_mut().chr_write(address, contents),
                _ => panic!("PPU unable to write address ${:x} to cartridge", address)
            },
            0x2000..=0x3EFF => {
                let mirror_mode = match self.cartridge {
                    Some(ref cart) => cart.borrow().mirroring,
                    _ => panic!("Unable to get cartridge mirror mode")
                };
                match mirror_mode {
                    0 => {
                        match address {
                            0x2000..=0x23FF => { self.nametables[(address as usize - 0x2000)] = contents; contents },
                            0x2400..=0x27FF => { self.nametables[(address as usize - 0x2400)] = contents; contents },
                            0x2800..=0x2BFF => { self.nametables[(address as usize - 0x2800)] = contents; contents },
                            0x2C00..=0x2FFF => { self.nametables[(address as usize - 0x2C00)] = contents; contents },
                            0x3000..=0x33FF => { self.nametables[(address as usize - 0x3000)] = contents; contents },
                            0x3400..=0x37FF => { self.nametables[(address as usize - 0x3400)] = contents; contents },
                            0x3800..=0x3BFF => { self.nametables[(address as usize - 0x3800)] = contents; contents },
                            0x3C00..=0x3EFF => { self.nametables[(address as usize - 0x3C00)] = contents; contents },
                            _ => contents
                        }
                    }
                    _ => panic!("Haven't dealt with mirroring")
                }
            },
            0x3F00..=0x3FFF => { self.palettes[(address % 32) as usize] = contents; contents },
            _ => panic!("PPU requested write outside of memory range: ${:x}", address)
        };
        result
    }
}