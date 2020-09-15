use crate::cartridge::Mapper;
use crate::cartridge::RomData;
use crate::cartridge::Mirror;

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

enum AddressingMode {
    Low,
    High
}

impl Mapper for MMC1 {
    fn prg_read(&self, address: u16) -> u8 {
        let val = match address {
            0x6000..=0x7FFF => self.mmc1_read_prg_ram(address - 0x6000),
            0x8000..=0xBFFF => self.mmc1_read_prg_rom(AddressingMode::Low, address - 0x8000),
            0xC000..=0xFFFF => self.mmc1_read_prg_rom(AddressingMode::High, address - 0xC000),
            _ => panic!("Unable to read address: {:04x}", address)
        };
        val
    }

    fn prg_write(&mut self, address: u16, contents: u8) {
        match address {
            0x6000..=0x7FFF => { self.mmc1_write_prg_ram(address - 0x6000, contents) },
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

    fn mirror_mode(&self) -> Mirror {
        let val: Mirror = match self.ctrl & 0b11 {
            0 => Mirror::OneScreenL,
            1 => Mirror::OneScreenH,
            2 => Mirror::Vertical,
            3 => Mirror::Horizontal,
            _ => panic!("No idea how this happened")
        };
        val
    }
}

impl MMC1 {
    fn mmc1_read_prg_ram(&self, address: u16) -> u8 {
        self.data.prg_ram[address as usize]
    }

    fn mmc1_write_prg_ram(&mut self, address: u16, contents: u8) {
        self.data.prg_ram[address as usize] = contents;
    }

    fn mmc1_read_prg_rom(&self, addressing_mode: AddressingMode, address: u16) -> u8 {
        let prg_mode = (self.ctrl & 0b1100) >> 2;
        let page = match prg_mode {
            0..=1 => match addressing_mode {
                AddressingMode::Low => (self.prg & !1) & 0b1111,
                AddressingMode::High => (self.prg | 1) & 0b1111
            },
            2 => match addressing_mode {
                AddressingMode::Low => 0,
                AddressingMode::High => self.prg
            },
            3 => match addressing_mode {
                AddressingMode::Low => self.prg,
                AddressingMode::High => (self.data.prg_rom.len() / 0x4000) as u8 - 1
            },
            _ => panic!("Invalid addressing mode")
        };
        self.data.prg_rom[(0x4000 * page as usize) + address as usize]
    }

    fn load_register(&mut self, address: u16, contents: u8) {
        if self.update_reg(contents) == true {
            match address {
                0x8000..=0x9FFF => { self.ctrl = self.shift_val },
                0xA000..=0xBFFF => { self.chr_0 = self.shift_val & 0b11111 },
                0xC000..=0xDFFF => { self.chr_1 = self.shift_val & 0b11111 },
                0xE000..=0xFFFF => { self.prg = self.shift_val & 0b1111 },
                _ => panic!("Address out of range! {:04x}", address)
            }
        }
    }

    fn update_reg(&mut self, contents: u8) -> bool {
        if self.shift_reg & 0x80 == 0x80 {
            self.reset_shift_reg();
        }
        else {
            self.shift_reg |= (contents & 1) << self.shift_reg_index;
            if self.shift_reg_index == 4 {
                self.shift_val = self.shift_reg;
                self.reset_shift_reg();
                return true
            }
            self.shift_reg_index += 1;
        }
        false
    }

    fn reset_shift_reg(&mut self) {
        self.shift_reg = 0;
        self.shift_reg_index = 0;
    }
}
