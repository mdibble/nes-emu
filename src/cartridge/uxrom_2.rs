use crate::cartridge::Mapper;
use crate::cartridge::RomData;
use crate::cartridge::Mirror;

pub struct UxROM {
    data: RomData,
    bank: u8
}

impl UxROM {
    pub fn new(mut data: RomData) -> UxROM {
        data.chr_ram.resize(0x2000, 0);

        UxROM {
            data: data,
            bank: 0
        }
    }
}

impl Mapper for UxROM {
    fn prg_read(&self, address: u16) -> u8 {
        let val = match address {
            0x8000..=0xBFFF => self.data.prg_rom[(0x4000 * self.bank as usize) + (address as usize - 0x8000)],
            0xC000..=0xFFFF => self.data.prg_rom[self.data.prg_rom.len() - 0x4000 + (address as usize - 0xC000)],
            _ => 0
        };
        val
    }

    fn prg_write(&mut self, address: u16, contents: u8) {
        match address {
            0x8000..=0xFFFF => self.bank = contents & 0b00001111,
            _ => panic!("Undesired write")
        }
    }

    fn chr_read(&self, address: u16) -> u8 {
        if self.data.header.chr_rom_size != 0 {  
            self.data.chr_rom[address as usize]
        }
        else {
            self.data.chr_ram[address as usize]
        }
    }

    fn chr_write(&mut self, address: u16, contents: u8) {
        self.data.chr_ram[address as usize] = contents;
    }

    fn mirror_mode(&self) -> Mirror {
        let val: Mirror = match self.data.header.mirror_mode {
            0 => Mirror::Horizontal,
            1 => Mirror::Vertical,
            _ => panic!("Mirror mode not implemented (UxROM)")
        };
        val
    }
}