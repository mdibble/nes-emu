use crate::cartridge::Mapper;
use crate::cartridge::RomData;
use crate::cartridge::Mirror;

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
    fn prg_read(&self, address: u16) -> u8 {
        let val = match address {
            0x6000..=0x7FFF => self.data.prg_ram[address as usize - 0x6000],
            0x8000..=0xBFFF => self.data.prg_rom[address as usize - 0x8000],
            0xC000..=0xFFFF => self.data.prg_rom[self.data.prg_rom.len() - 0x4000 + (address as usize - 0xC000)],
            _ => panic!("Invalid read! (0x{:x})", address)
        };
        val
    }
    fn prg_write(&mut self, address: u16, contents: u8) {
        match address {
            0x6000..=0x7FFF => self.data.prg_ram[address as usize - 0x6000] = contents,
            _ => panic!("Invalid write!")
        };
    }
    fn chr_read(&self, address: u16) -> u8 {
        if self.data.header.chr_rom_size == 0 {
            self.data.chr_ram[address as usize]
        }
        else {
            self.data.chr_rom[address as usize]
        }
    }
    fn chr_write(&mut self, address: u16, contents: u8) {
        if self.data.header.chr_rom_size == 0 {
            self.data.chr_ram[address as usize] = contents;
        }
    }

    fn mirror_mode(&self) -> Mirror {
        let val: Mirror = match self.data.header.mirror_mode {
            0 => Mirror::Horizontal,
            1 => Mirror::Vertical,
            _ => panic!("Mirror mode not implemented (NROM)")
        };
        val
    }
}