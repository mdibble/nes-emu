mod mapper;
mod nrom_0;
mod rom_data;
mod rom_header;

use self::mapper::Mapper;
use self::nrom_0::NROM;
use self::rom_data::RomData;

pub struct Cartridge {
    mapper: Box<dyn Mapper>
}

impl Cartridge {
    pub fn new(dump: Vec<u8>) -> Cartridge {
        let rom_data = RomData::new(dump);

        let mapper: Box<dyn Mapper> = match rom_data.header.mapper_id {
            0 => Box::new(NROM::new(rom_data)),
            _ => panic!("Mapper isn't supported")
        };
    
        let cartridge = Cartridge {
            mapper: mapper
        };
        cartridge
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mapper.read(address)
    }

    pub fn write(&mut self, address: u16, contents: u8) -> u8 {
        self.mapper.write(address, contents)
    }
}