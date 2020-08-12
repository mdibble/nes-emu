use crate::cpu::CPU;
use crate::cartridge::Cartridge;

pub struct NES {
    cpu: CPU
}

impl NES {
    pub fn new() -> NES {
        let nes = NES {
            cpu: CPU::new()
        };
        nes
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cpu.bus.insert_cartridge(cartridge);
    }

    pub fn cycle(&mut self) {
        self.cpu.tick();
    }
}