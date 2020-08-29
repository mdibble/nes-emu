use crate::cartridge::Cartridge;
use crate::ppu::PPU;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bus {
    memory: [u8; 0x10000],
    pub ppu: PPU,
    // apu: APU
    pub cartridge: Rc<RefCell<Cartridge>>
}

impl Bus {
    pub fn new(cart_data: Vec<u8>) -> Bus {
        let bus = Bus {
            memory: [0; 0x10000],
            ppu: PPU::new(),
            cartridge: Rc::new(RefCell::new(Cartridge::new(cart_data)))
        };
        bus
    }

    pub fn reset(&mut self) {
        self.ppu.assign_cartridge(self.cartridge.clone());
        self.ppu.reset();
    }

    pub fn get_memory(&mut self, address: u16) -> u8 {
        let result: u8;
        
        match address {
            0x0000..=0x1FFF => {
                // CPU memory
                result = self.memory[address as usize % 0x800]
            }
            0x2000..=0x3FFF => {
                // PPU registers
                // result = self.memory[address as usize % 0x8]
                result = self.ppu.get_reg(address);
            }
            0x4000..=0x4017 => {
                // Input and APU
                result = self.memory[address as usize]
            }
            0x4018..=0x401F => {
                // Input and APU continued
                result = self.memory[address as usize]
            }
            0x4020..=0xFFFF => {
                // Cartridge space
                result = self.cartridge.borrow().prg_read(address);
            }
        }
        result
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) -> u8 {
        match address {
            0x0000..=0x1FFF => {
                // CPU memory
                self.memory[address as usize % 0x800] = contents;
            }
            0x2000..=0x3FFF => {
                // PPU registers
                self.ppu.write_reg(address, contents);
            }
            0x4000..=0x4017 => {
                // Input and APU
                self.memory[address as usize] = contents;
            }
            0x4018..=0x401F => {
                // Input and APU continued
                self.memory[address as usize] = contents;
            }
            0x4020..=0xFFFF => {
                // Cartridge space
                self.cartridge.borrow_mut().prg_write(address, contents);
                self.memory[address as usize] = contents;
            }
        }
        contents
    }
}