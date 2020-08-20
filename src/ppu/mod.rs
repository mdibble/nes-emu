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
        match address {
            0x2000 => println!("PPU register 0x2000"),
            0x2001 => println!("PPU register 0x2001"),
            0x2002 => println!("PPU register 0x2002"),
            0x2003 => println!("PPU register 0x2003"),
            0x2004 => println!("PPU register 0x2004"),
            0x2005 => println!("PPU register 0x2005"),
            0x2006 => println!("PPU register 0x2006"),
            0x2007 => println!("PPU register 0x2007"),
            _ => panic!("No register at this location! ${:x}", address)
        }
        0
    }

    pub fn write_reg(&self, address: u16, contents: u8) -> u8 {
        match address {
            0x2000 => println!("PPU register 0x2000 = {:x}", contents),
            0x2001 => println!("PPU register 0x2001 = {:x}", contents),
            0x2002 => println!("PPU register 0x2002 = {:x}", contents),
            0x2003 => println!("PPU register 0x2003 = {:x}", contents),
            0x2004 => println!("PPU register 0x2004 = {:x}", contents),
            0x2005 => println!("PPU register 0x2005 = {:x}", contents),
            0x2006 => println!("PPU register 0x2006 = {:x}", contents),
            0x2007 => println!("PPU register 0x2007 = {:x}", contents),
            _ => panic!("No register at this location! ${:x}", address)
        }
        0
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_memory(&mut self, address: u16, contents: u8) -> u8 {
        self.memory[address as usize] = contents;
        contents
    }
}