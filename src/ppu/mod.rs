use crate::bus::Bus;
 
pub struct PPU {
    // bus: &Bus,
    memory: [u8; 0x4000], // $0000-$3FFF
    scanline: u16, // 260, y-axis
    cycle: u16, // 340, x-axis
    frame_complete: bool
}

impl PPU {
    pub fn new() -> PPU {
        let ppu = PPU {
            // bus reference
            memory: [0; 0x4000],
            scanline: 0,
            cycle: 0,
            frame_complete: false
        };
        ppu
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 260 {
                self.scanline -= 1;
                self.frame_complete = true;
            }
        }
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn ppu_write(&mut self, address: u16, contents: u8) -> u8 {
        self.memory[address as usize] = contents;
        contents
    }

    // pub fn cpu_read(&self, address: u16) -> u8 {
    //     self.bus.get_memory(address as usize)
    // }

    // pub fn cpu_write(&mut self, address: u16, contents: u8) -> u8 {
    //     self.bus.write_memory(address as usize, contents);
    //     contents
    // }
}