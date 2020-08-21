use crate::cartridge::Mapper;
use crate::cartridge::RomData;

pub struct NROM {
    data: RomData
}

impl NROM {
    pub fn new(data: RomData) -> NROM {
        NROM {
            data: data
        }
    }
}

impl Mapper for NROM {
    fn read(&self, address: u16) -> u8 {
        let val = match address {
            0x6000..=0x7FFF => self.data.prg_ram[address as usize - 0x6000],
            0x8000..=0xBFFF => self.data.prg_rom[address as usize - 0x8000],
            0xC000..=0xFFFF => self.data.prg_rom[self.data.prg_rom.len() - 0x4000 + (address as usize - 0xC000)],
            _ => panic!("Invalid read!")
        };
        val
    }
    fn write(&mut self, address: u16, contents: u8) -> u8 {
        match address {
            0x6000..=0x7FFF => self.data.prg_ram[address as usize - 0x6000] = contents,
            0x8000..=0xBFFF => self.data.prg_rom[address as usize - 0x8000] = contents,
            0xC000..=0xFFFF => {
                let end = self.data.prg_rom.len();
                self.data.prg_rom[end - 0x4000 + (address as usize - 0xC000)] = contents;
            }, 
            _ => panic!("Invalid read!")
        };
        contents
    }
}