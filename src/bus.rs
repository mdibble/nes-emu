use crate::cartridge::Cartridge;

pub struct Bus {
    memory: [u8; 0xFFFF],
    // ppu: PPU,
    // apu: APU
    cartridge: Cartridge
}

impl Bus {
    pub fn new() -> Bus {
        let mut bus = Bus {
            memory: [0; 0xFFFF],
            cartridge: Cartridge::new()
        };
        bus
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize % 0xFFFF]
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) -> u8 {
        self.memory[address as usize] = contents;
        contents
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge;
        self.cartridge.get_info();
    }
}