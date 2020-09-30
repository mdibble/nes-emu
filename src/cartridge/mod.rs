mod mapper;
mod nrom_0;
mod mmc1_1;
mod uxrom_2;
mod rom_data;
mod rom_header;

use self::mapper::Mapper;
use self::nrom_0::NROM;
use self::mmc1_1::MMC1;
use self::uxrom_2::UxROM;
use self::rom_data::RomData;

pub enum Mirror {
    Horizontal,
    Vertical,
    OneScreenL,
    OneScreenH
}

pub struct Cartridge {
    pub mapper: Box<dyn Mapper>
}

impl Cartridge {
    pub fn new(dump: Vec<u8>) -> Cartridge {
        let rom_data = RomData::new(dump);

        let mapper: Box<dyn Mapper> = match rom_data.header.mapper_id {
            0 => Box::new(NROM::new(rom_data)),
            1 => Box::new(MMC1::new(rom_data)),
            2 => Box::new(UxROM::new(rom_data)),
            _ => panic!("Mapper isn't supported")
        };
    
        let cartridge = Cartridge {
            mapper: mapper
        };
        cartridge
    }

    pub fn prg_read(&self, address: u16) -> u8 {
        self.mapper.prg_read(address)
    }

    pub fn prg_write(&mut self, address: u16, contents: u8) {
        self.mapper.prg_write(address, contents)
    }

    pub fn chr_read(&self, address: u16) -> u8 {
        self.mapper.chr_read(address)
    }

    pub fn chr_write(&mut self, address: u16, contents: u8) {
        self.mapper.chr_write(address, contents)
    }

    pub fn mirror_mode(&self) -> Mirror {
        self.mapper.mirror_mode()
    }
}