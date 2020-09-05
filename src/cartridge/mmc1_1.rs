use crate::cartridge::Mapper;
use crate::cartridge::RomData;

pub struct MMC1 {
    data: RomData,
    shift_reg: u8,
    ctrl_reg: u8
}

impl MMC1 {
    pub fn new(data: RomData) -> MMC1 {
        MMC1 {
            data: data,
            shift_reg: 0,
            ctrl_reg: 0
        }
    }
}

impl Mapper for MMC1 {
    fn prg_read(&self, address: u16) -> u8 {
        0
    }

    fn prg_write(&mut self, address: u16, contents: u8) {
    }

    fn chr_read(&self, address: u16) -> u8 {
        0
    }

    fn chr_write(&mut self, address: u16, contents: u8) {
    }
}