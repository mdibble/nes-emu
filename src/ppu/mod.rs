mod registers;
mod colors;

use colors::RGB;

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
    reg_ppu_scroll: u8,     // $2005
    reg_ppu_addr: u8,       // $2006
    reg_ppu_data: u8,       // $2007

    pub trigger_nmi: bool,

    cartridge: Option<Rc<RefCell<Cartridge>>>,

    pub display: Vec<RGB>
}

impl PPU {
    pub fn new() -> PPU {
        let ppu = PPU {
            nametables: [0; 0x800],
            palettes: [0; 0x20],
            oam_memory: [0; 0x100],
            scanline: 0,
            cycle: 0,
            reg_ppu_ctrl: 0b00000000,
            reg_ppu_mask: 0b00000000,
            reg_ppu_status: 0b00000000,
            reg_oam_addr: 0b00000000,
            reg_oam_data: 0b00000000,
            reg_ppu_scroll: 0b00000000,
            reg_ppu_addr: 0b00000000,
            reg_ppu_data: 0b00000000,

            trigger_nmi: false,

            cartridge: None,
            display: vec![RGB{ r: 0, g: 0, b: 0 }; 256 * 240]
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
        self.reg_ppu_scroll = 0b00000000;
        self.reg_ppu_addr = 0b00000000;
        self.reg_ppu_data = 0b00000000; 
    }

    pub fn assign_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
    }

    pub fn render_pixel(&self) {
        println!("Pixel should be rendered");
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
            }
        }

        // Information of current scanline/cycle
        let enable_rendering = self.reg_ppu_mask & 0b00010000 != 0 || self.reg_ppu_mask & 0b00001000 != 0;
        let visible_line = self.scanline <= 240;
        let visible_cycle = self.cycle >= 1 && self.cycle <= 256;
        let render_line = visible_line || self.scanline == 261;

        // Start of drawing

        if enable_rendering {
            if visible_line && visible_cycle {
                self.render_pixel();
            }

        }

        // End of drawing

        // VBlank
        if self.scanline == 241 && self.cycle == 1 {
            self.reg_ppu_status |= 0b10000000;  
            println!("VBLANK has begun - CTRL: {:08b}", self.reg_ppu_ctrl);
            if self.reg_ppu_ctrl & 0b10000000 == 0b10000000 {
                self.trigger_nmi = true;
                panic!("Interrupt Requested"); // placeholder
            }
        }

        //VBlank off
        if self.scanline == 261 && self.cycle == 1 {
            println!("VBLANK has ended");
            self.reg_ppu_status &= 0b01111111;  
            self.reg_ppu_status &= 0b10111111;  // sprite 0 hit
            self.reg_ppu_status &= 0b11011111;  // sprite overflow
        }

    }

    pub fn get_reg(&mut self, address: u16) -> u8 {
        println!("PPU register accessed through ${:x}: read", address);

        let result = match address {
            0x2002 => self.read_ppu_status(),
            0x2004 => self.read_oam_data(),
            0x2007 => self.read_ppu_data(),
            _ => panic!("No register at this location! ${:x}", address)
        };
        result
    }

    pub fn write_reg(&mut self, address: u16, contents: u8) -> u8 {
        println!("PPU register accessed through ${:x}: write", address);

        self.reg_ppu_status = self.reg_ppu_status & 0b11100000;
        let new_val = contents & 0x1F;
        self.reg_ppu_status = self.reg_ppu_status | new_val;

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
        println!("PPU memory accessed: read");
        let result = match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref cart) => cart.borrow().read(address),
                _ => panic!("PPU unable to read address ${:x} from cartridge", address)
            }
            0x2000..=0x3EFF => self.nametables[address as usize - 0x2000],
            0x3F00..=0x3FFF => self.palettes[address as usize - 0x3F00],
            _ => panic!("PPU requested read outside of memory range: ${:x}", address)
        };
        result
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) -> u8 {
        println!("PPU memory accessed: write");
        let result = match address {
            0x0000..=0x1FFF => match self.cartridge {
                Some(ref cart) => cart.borrow_mut().write(address, contents),
                _ => panic!("PPU unable to write address ${:x} to cartridge", address)
            }
            0x2000..=0x3EFF => { self.nametables[address as usize - 0x2000] = contents; contents },
            0x3F00..=0x3FFF => { self.palettes[address as usize - 0x3F00] = contents; contents },
            _ => panic!("PPU requested write outside of memory range: ${:x}", address)
        };
        result
    }
}