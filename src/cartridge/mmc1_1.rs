use crate::cartridge::Mapper;
use crate::cartridge::RomData;

pub struct MMC1 {
    data: RomData,

    shift_reg: u8,
    shift_reg_index: u8,
    shift_val: u8,

    ctrl: u8,
    chr_0: u8,
    chr_1: u8,
    prg: u8
}

impl MMC1 {
    pub fn new(data: RomData) -> MMC1 {
        MMC1 {
            data: data,

            shift_reg: 0,
            shift_reg_index: 0,
            shift_val: 0,

            ctrl: 0,
            chr_0: 0,
            chr_1: 0,
            prg: 0
        }
    }
}

impl Mapper for MMC1 {
    fn prg_read(&self, address: u16) -> u8 {
        let val = match address {
            0x6000..=0x7FFF => self.data.prg_ram[address as usize - 0x6000],
            0x8000..=0xBFFF => self.data.prg_rom[address as usize - 0x8000],
            0xC000..=0xFFFF => self.data.prg_rom[(self.data.prg_rom.len() - 0x4000) + (address as usize - 0xC000)],
            _ => panic!("Unable to read address: {:04x}", address)
        };
        val
    }

    fn prg_write(&mut self, address: u16, contents: u8) {
        match address {
            0x6000..=0x7FFF => { self.data.prg_ram[address as usize - 0x6000] = contents; },
            0x8000..=0xFFFF => { self.load_register(address, contents); }
            _ => panic!("Unable to write to address: {:04x}", address)
        }
    }

    fn chr_read(&self, address: u16) -> u8 {
        self.data.chr_rom[address as usize]
    }

    fn chr_write(&mut self, address: u16, contents: u8) {
        self.data.chr_ram[address as usize] = contents;
    }

    
}

impl MMC1 {
    fn load_register(&mut self, address: u16, contents: u8) {
        self.update_reg(contents);

        match address {
            0x8000..=0x9FFF => { self.ctrl = self.shift_val },
            0xA000..=0xBFFF => { self.chr_0 = self.shift_val & 0b11111 },
            0xC000..=0xDFFF => { self.chr_1 = self.shift_val & 0b11111 },
            0xE000..=0xFFFF => { self.prg = self.shift_val & 0b1111 },
            _ => panic!("Address out of range! {:04x}", address)
        }
    }

    fn update_reg(&mut self, contents: u8) {
        if self.shift_reg & 0x80 == 0x80 {
            self.reset_shift_reg();
        }
        else {
            self.shift_reg |= (contents & 1) << self.shift_reg_index;
            if self.shift_reg_index == 4 {
                self.shift_val = self.shift_reg;
                self.reset_shift_reg();
            }
            else {
                self.shift_reg_index += 1;
            }
        }
        self.shift_val = 0;
    }

    fn reset_shift_reg(&mut self) {
        self.shift_reg = 0;
        self.shift_reg_index = 0;
    }
}
