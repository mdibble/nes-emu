use crate::cartridge::Cartridge;

pub struct Bus {
    memory: [u8; 0x800],
    // ppu: PPU,
    // apu: APU
    cartridge: Cartridge
}

impl Bus {
    pub fn new() -> Bus {
        let bus = Bus {
            memory: [0; 0x800],
            cartridge: Cartridge::new()
        };
        bus
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge;
        self.cartridge.get_info();
    }
}