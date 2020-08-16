use crate::cpu::CPU;
use crate::cartridge::Cartridge;

use std::fs;


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

    pub fn cpu_test(&mut self, path: &str) {
        let payload = fs::read(path);
        let payload = match payload {
            Ok(g) => g,
            Err(_) => panic!("Error! No file was found at location")
        };
        let length_of = 0xFFFF - 0xBFFF;
        for i in 0..length_of {
            self.cpu.bus.write_memory(0xC000 + i as u16, payload[i + 16]);
        }
        
        self.cpu.bus.write_memory(0xFFFE, 0x00); 
        self.cpu.bus.write_memory(0xFFFF, 0xC0);
        self.cpu.reset(); 
    }

    pub fn cycle(&mut self) {
        self.cpu.tick();
        self.cpu.bus.ppu.tick();
        self.cpu.bus.ppu.tick();
        self.cpu.bus.ppu.tick();
    }
}