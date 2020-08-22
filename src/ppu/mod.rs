mod registers;

use crate::cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    nametables: [u8; 0x800],
    palettes: [u8; 0x20],
    oam_memory: [u8; 0x100],
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

    cartridge: Option<Rc<RefCell<Cartridge>>>
}

impl PPU {
    pub fn new() -> PPU {
        let ppu = PPU {
            // bus reference
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

            cartridge: None
        };
        ppu
    }

    pub fn assign_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge);
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