mod registers;
mod colors;

use colors::SYS_COLORS;

use crate::cartridge::Cartridge;
use crate::cartridge::Mirror;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    nametables: [u8; 0x800],
    palettes: [u8; 0x20],
    pub oam_memory: [u8; 0x100],
    pub secondary_oam: [u8; 0x20],
    pub scanline: u16, // 260, y-axis
    pub cycle: u16, // 340, x-axis

    reg_ppu_ctrl: u8,       // $2000
    reg_ppu_mask: u8,       // $2001
    reg_ppu_status: u8,     // $2002
    reg_oam_addr: u8,       // $2003

    data_buffer: u8,

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

    shift_reg_sprite_lo: [u8; 8],
    shift_reg_sprite_hi: [u8; 8],
    sprite_latch: [u8; 8],
    sprite_counter: [u8; 8],

    queue_sprite_zero: bool,
    sprite_zero_in_line: bool,

    cartridge: Option<Rc<RefCell<Cartridge>>>,

    pub nmi_occurred: bool,
    pub nmi_output: bool,
    pub trigger_nmi: bool,

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
            secondary_oam: [0; 0x20],
            scanline: 0,
            cycle: 0, // maybe should be 341?
            reg_ppu_ctrl: 0b00000000,
            reg_ppu_mask: 0b00000000,
            reg_ppu_status: 0b00000000,
            reg_oam_addr: 0b00000000,

            data_buffer: 0x00,

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

            shift_reg_sprite_lo: [0; 8],
            shift_reg_sprite_hi: [0; 8],
            sprite_latch: [0; 8],
            sprite_counter: [0; 8],

            queue_sprite_zero: false,
            sprite_zero_in_line: false,

            cartridge: None,

            nmi_occurred: false,
            nmi_output: false,
            trigger_nmi: false,

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
        self.reg_oam_addr = 0b00000000;
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
        let shift = ((self.vram_address >> 4) & 4) | (self.vram_address & 2);
        self.fetched_at = (self.get_memory(address) >> shift) & 3;
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

    pub fn eval_sprites(&mut self) {
        let sprite_size = if self.reg_ppu_ctrl & 0b00100000 == 0b00100000 { 16 } else { 8 };
        let mut spr_count = 0;

        for n in 0..64 {
            let y = self.oam_memory[n * 4] as u16; // u16 may not work here

            if self.scanline >= y && self.scanline - y < sprite_size {
                for m in 0..4 {
                    self.secondary_oam[spr_count * 4 + m] = self.oam_memory[n * 4 + m];
                }

                if n == 0 {
                    self.queue_sprite_zero = true;
                }

                spr_count += 1;
            }

            if spr_count == 8 {
                self.reg_ppu_status |= 0b00100000; // Hardware bugs associated with this
                break;
            }
        }
    }

    pub fn fetch_sprites(&mut self) {
        self.shift_reg_sprite_lo = [0; 8];
        self.shift_reg_sprite_hi = [0; 8];
        self.sprite_latch = [0; 8];
        self.sprite_counter = [0; 8];

        let mut sprite_num = 0;
        for i in 0..8 {
            if self.secondary_oam[(i * 4) + 0] == 0xFF && self.secondary_oam[(i * 4) + 1] == 0xFF && self.secondary_oam[(i * 4) + 2] == 0xFF && self.secondary_oam[(i * 4) + 3] == 0xFF {
                break;
            }
            else {
                sprite_num += 1;
            }
        }

        for i in 0..sprite_num {
            let byte_0 = self.secondary_oam[i * 4 + 0]; // Y position
            let byte_1 = self.secondary_oam[i * 4 + 1]; // PT tile
            let byte_2 = self.secondary_oam[i * 4 + 2]; // Orientation, priority, palette
            let byte_3 = self.secondary_oam[i * 4 + 3]; // X position

            let sprite_size = if self.reg_ppu_ctrl & 0b00100000 == 0b00100000 { 16 } else { 8 };

            let row = self.scanline - byte_0 as u16;
            
            let mut address;

            if sprite_size == 8 {
                let anchor: u16 = if ((self.reg_ppu_ctrl >> 3) & 1) == 1 { 0x1000 } else { 0x0 };
                address = anchor + (byte_1 as u16 * 16);
            }

            else {
                let anchor: u16 = if (byte_1 & 1) == 1 { 0x1000 } else { 0x0 };
                address = anchor + ((byte_1 as u16 & 0b11111110) * 16);
            }

            if byte_2 & 0x80 != 0x80 {
                address += row;
            }   
            else {
                address += sprite_size - 1 - row;
            }

            let mut sr_temp_lo = self.get_memory(address);
            let mut sr_temp_hi = self.get_memory(address + 8);
            
            if byte_2 & 0x40 == 0x40 {
                let old_lo = sr_temp_lo;
                let old_hi = sr_temp_hi;

                sr_temp_lo = 0;
                sr_temp_hi = 0;

                for i in 0..8 {
                    if (old_lo >> i) & 1 == 1 {
                        sr_temp_lo |= 1 << (7 - i);
                    }
                    if (old_hi >> i) & 1 == 1 {
                        sr_temp_hi |= 1 << (7 - i);
                    }
                }
            }

            self.shift_reg_sprite_lo[i] = sr_temp_lo;
            self.shift_reg_sprite_hi[i] = sr_temp_hi;

            self.sprite_latch[i] = byte_2;
            self.sprite_counter[i] = byte_3;
        }
    }

    pub fn render_pixel(&mut self) {
        let row = self.scanline;
        let col = self.cycle - 1;
        
        let bg_shift = 15 - self.x_scroll;
        let palette_shift = 7 - self.x_scroll;
        
        let pixel = ((self.shift_reg_pt_hi >> bg_shift) & 1) << 1 | ((self.shift_reg_pt_lo >> bg_shift) & 1);
        let palette = ((self.shift_reg_palette_hi >> palette_shift) & 1) << 1 | ((self.shift_reg_palette_lo >> palette_shift) & 1);

        let new_palette = self.get_memory(self.get_palette_address(palette, pixel));

        self.display[(row as usize * 256 * 3) + (col * 3) as usize + 0] = SYS_COLORS[new_palette as usize].r;
        self.display[(row as usize * 256 * 3) + (col * 3) as usize + 1] = SYS_COLORS[new_palette as usize].g;
        self.display[(row as usize * 256 * 3) + (col * 3) as usize + 2] = SYS_COLORS[new_palette as usize].b;

        for i in 0..8 {
            if self.sprite_counter[i] == 0 && ((self.shift_reg_sprite_lo[i] & 0x80) != 0 || (self.shift_reg_sprite_hi[i] & 0x80) != 0) {
                let spr_pixel = (self.shift_reg_sprite_hi[i] >> 7) << 1 | self.shift_reg_sprite_lo[i] >> 7;
                let spr_palette = (self.sprite_latch[i] & 0x3) + 4;

                let spr_new_palette = self.get_memory(self.get_palette_address(spr_palette, spr_pixel as u16));

                if i == 0 && self.sprite_zero_in_line {
                    if pixel != 0 && spr_pixel != 0 {
                        self.reg_ppu_status |= 0x40;
                    }
                }
                
                if pixel == 0 || (self.sprite_latch[i] >> 5) & 1 == 0 {
                    self.display[(row as usize * 256 * 3) + (col * 3) as usize + 0] = SYS_COLORS[spr_new_palette as usize].r;
                    self.display[(row as usize * 256 * 3) + (col * 3) as usize + 1] = SYS_COLORS[spr_new_palette as usize].g;
                    self.display[(row as usize * 256 * 3) + (col * 3) as usize + 2] = SYS_COLORS[spr_new_palette as usize].b;
                    break;
                }
            }
        }

        if self.reg_ppu_mask & 0b00010000 == 0b00010000 {
            for i in 0..8 {
                if self.sprite_counter[i] == 0 {
                    self.shift_reg_sprite_lo[i] <<= 1;
                    self.shift_reg_sprite_hi[i] <<= 1;
                }
            }
    
            for i in 0..8 {
                if self.sprite_counter[i] > 0 {
                    self.sprite_counter[i] -= 1;
                }
            }
        }   
    }

    pub fn get_palette_address(&self, palette: u8, pixel: u16) -> u16 {
        if pixel == 0 {
            0x3F00
        }
        else {
            0x3F00 + (palette as u16 * 4) + pixel // May need revision
        }
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                // if self.even_frame {
                //     self.cycle += 1;
                //     self.even_frame = !self.even_frame;
                // }
            }
        }

        // Information of current scanline/cycle
        let enable_rendering = (self.reg_ppu_mask & 0b00011000) != 0;
        let visible_scanline = self.scanline <= 239;
        let prerender_scanline = self.scanline == 261;
        let visible_cycle = self.cycle >= 1 && self.cycle <= 256;
        let fetch_cycle = self.cycle >= 321 && self.cycle <= 336;

        // Start of drawing

        if self.cycle == 0 && self.queue_sprite_zero {
            self.sprite_zero_in_line = true;
            self.queue_sprite_zero = false;
        }

        if self.cycle == 256 {
            self.sprite_zero_in_line = false;
        }

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

        if enable_rendering && visible_scanline {
            match self.cycle {
                64 => { self.secondary_oam = [0xFF; 0x20] },
                256 => { self.eval_sprites() },
                340 => { self.fetch_sprites() },
                _ => { }
            }
        }

        // End of drawing

        // VBlank
        if self.scanline == 241 && self.cycle == 1 {
            self.draw = true; // ready to draw to canvas
            self.reg_ppu_status |= 0b10000000;
            if (self.reg_ppu_ctrl & 0b10000000) == 0b10000000 {
                self.nmi_output = true;
            }
            self.nmi_occurred = true;
            self.update_nmi_status();
            
        }

        // VBlank off
        if self.scanline == 261 && self.cycle == 1 {
            self.nmi_occurred = false;
            self.update_nmi_status();
            self.reg_ppu_status &= 0b00011111;
        }
    }

    pub fn update_nmi_status(&mut self) {
        if self.nmi_output && self.nmi_occurred {
            self.trigger_nmi = true;
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
        self.vram_address &= 0b0111101111100000;
        self.vram_address |= self.temp_address & 0b0000010000011111;
    }

    pub fn y_copy(&mut self) {
        self.vram_address &= 0b0000010000011111;
        self.vram_address |= self.temp_address & 0b0111101111100000;
    }

    pub fn get_reg(&mut self, address: u16) -> u8 {
        let result = match address {
            0x2002 => self.read_ppu_status(),
            0x2004 => self.read_oam_data(),
            0x2007 => self.read_ppu_data(),
            _ => { println!("ALERT: Register ${:04x} read", address); 0 }
        };
        result
    }

    pub fn write_reg(&mut self, address: u16, contents: u8) {
        self.reg_ppu_status &= 0b11100000;
        self.reg_ppu_status |= contents & 0b00011111;
        match address {
            0x2000 => self.write_ppu_ctrl(contents),
            0x2001 => self.write_ppu_mask(contents),
            0x2003 => self.write_oam_addr(contents),
            0x2004 => self.write_oam_data(contents),
            0x2005 => self.write_ppu_scroll(contents),
            0x2006 => self.write_ppu_addr(contents),
            0x2007 => self.write_ppu_data(contents),
            _ => panic!("No register at this location! ${:x}", address)
        }
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        let result = match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref cart) => cart.borrow().chr_read(address),
                _ => panic!("PPU unable to read address ${:x} from cartridge", address)
            }
            0x2000..=0x3EFF => {
                let mirror_mode = match self.cartridge {
                    Some(ref cart) => cart.borrow().mirror_mode(),
                    _ => panic!("Unable to get cartridge mirror mode")
                };
                let location = (address % 0x1000) + 0x2000;
                match mirror_mode {
                    Mirror::Horizontal => {
                        match location {
                            0x2000..=0x23FF => self.nametables[(location as usize - 0x2000)],
                            0x2400..=0x27FF => self.nametables[(location as usize - 0x2400)],
                            0x2800..=0x2BFF => self.nametables[(location as usize - 0x2400)],
                            0x2C00..=0x2FFF => self.nametables[(location as usize - 0x2800)],
                            _ => panic!("Invalid")
                        }
                    }
                    Mirror::Vertical => {
                        match location {
                            0x2000..=0x23FF => self.nametables[(location as usize - 0x2000)],
                            0x2400..=0x27FF => self.nametables[(location as usize - 0x2000)],
                            0x2800..=0x2BFF => self.nametables[(location as usize - 0x2800)],
                            0x2C00..=0x2FFF => self.nametables[(location as usize - 0x2800)],
                            _ => panic!("Invalid")
                        }
                    }
                    _ => panic!("Invalid mirror mode")
                }
                
            },
            0x3F00..=0x3FFF => {
                let mut palette_address = address % 32;
                if palette_address >= 16 && palette_address % 4 == 0 {
                    palette_address -= 16;
                }
                self.palettes[palette_address as usize]
            }
            _ => panic!("PPU requested read outside of memory range: ${:x}", address)
        };
        result
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) {
        match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref cart) => cart.borrow_mut().chr_write(address, contents),
                _ => panic!("PPU unable to write address ${:x} to cartridge", address)
            },
            0x2000..=0x3EFF => {
                let mirror_mode = match self.cartridge {
                    Some(ref cart) => cart.borrow().mirror_mode(),
                    _ => panic!("Unable to get cartridge mirror mode")
                };
                let location = (address % 0x1000) + 0x2000;
                let new_location: u16;
                match mirror_mode {
                    Mirror::Horizontal => {
                        new_location = match location {
                            0x2000..=0x23FF => location - 0x2000,
                            0x2400..=0x27FF => location - 0x2400,
                            0x2800..=0x2BFF => location - 0x2400,
                            0x2C00..=0x2FFF => location - 0x2800,
                            _ => 0
                        };
                    },
                    Mirror::Vertical => {
                        new_location = match location {
                            0x2000..=0x23FF => location - 0x2000,
                            0x2400..=0x27FF => location - 0x2000,
                            0x2800..=0x2BFF => location - 0x2800,
                            0x2C00..=0x2FFF => location - 0x2800,
                            _ => 0
                        };
                    },
                    _ => panic!("Unable to get cartridge mirror mode")
                }
                
                self.nametables[new_location as usize] = contents;
            },
            0x3F00..=0x3FFF => {
                let mut palette_address = address % 32;
                if palette_address >= 16 && palette_address % 4 == 0 {
                    palette_address -= 16;
                }
                self.palettes[palette_address as usize] = contents;
            }
            _ => panic!("PPU requested write outside of memory range: ${:x}", address)
        };
    }
}