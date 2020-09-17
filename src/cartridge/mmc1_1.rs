use crate::cartridge::Mapper;
use crate::cartridge::RomData;
use crate::cartridge::Mirror;

pub struct MMC1 {
    data: RomData,

    shift_reg: u8,
    ctrl: u8,
    chr_0: u8,
    chr_1: u8,
    prg: u8
}

impl MMC1 {
    pub fn new(mut data: RomData) -> MMC1 {
        data.prg_ram.resize(0x8000, 0);
        data.chr_ram.resize(0x2000, 0);

        MMC1 {
            data: data,

            shift_reg: 0x10,
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
            0x8000..=0xFFFF => { self.update_reg(address, contents); }
            _ => panic!("Unable to write to address: {:04x}", address)
        }
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
                AddressingMode::Low => (self.prg & 0xFE),
                AddressingMode::High => (self.prg | 0x01)
            },
            2 => match addressing_mode {
                AddressingMode::Low => 0,
                AddressingMode::High => self.prg
            },
            3 => match addressing_mode {
                AddressingMode::Low => self.prg,
                AddressingMode::High => self.data.header.prg_rom_size - 1
            },
            _ => panic!("Invalid addressing mode")
        };
        
        self.data.prg_rom[(0x4000 * page as usize) + address as usize]
    }

    fn update_reg(&mut self, address: u16, contents: u8) {
        if contents & 0x80 == 0x80 {
            self.shift_reg = 0x10;
            self.ctrl |= 0x0C;
        }
        else {
            let complete = (self.shift_reg & 1) == 1;
            self.shift_reg >>= 1;
            self.shift_reg |= (contents & 1) << 4;
            if complete {
                self.load_register(address);
                self.shift_reg = 0x10;
            }
        }
    }

    fn load_register(&mut self, address: u16) {
        match address {
                0x8000..=0x9FFF => { self.ctrl = self.shift_reg & 0b11111 },
                0xA000..=0xBFFF => { self.chr_0 = self.shift_reg & 0b11111 },
                0xC000..=0xDFFF => { self.chr_1 = self.shift_reg & 0b11111 },
                0xE000..=0xFFFF => { self.prg = self.shift_reg & 0b1111 },
                _ => panic!("Address out of range! {:04x}", address)
        }
    }
}
