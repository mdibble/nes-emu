use crate::cartridge::Cartridge;
use crate::ppu::PPU;

pub struct Bus {
    memory: [u8; 0x10000],
    pub ppu: PPU,
    // apu: APU
    cartridge: Cartridge
}

impl Bus {
    pub fn new() -> Bus {
        let bus = Bus {
            memory: [0; 0x10000],
            ppu: PPU::new(),
            cartridge: Cartridge::new()
        };
        bus
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        let result: u8;
        
        match address {
            0x0000..=0x1FFF => {
                // CPU memory
                result = self.memory[address as usize % 0x800]
            }
            0x2000..=0x3FFF => {
                // PPU registers
                result = self.memory[address as usize % 0x8]
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
                result = self.memory[address as usize]
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
                self.memory[address as usize % 0x8] = contents;
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
                self.memory[address as usize] = contents;
            }
        }

        self.memory[address as usize] = contents;
        contents
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge;
        self.cartridge.get_info();
    }
}