use crate::cpu::CPU;
use crate::bus::Bus;
use crate::cartridge::Cartridge;

pub struct NES {
    cpu: CPU,
    bus: Bus
}

impl NES {
    pub fn new() -> NES {
        let nes = NES {
            cpu: CPU::new(),
            bus: Bus::new()
        };
        nes
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.load_cartridge(cartridge);
    }
}