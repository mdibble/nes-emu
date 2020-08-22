use super::rom_header::RomHeader;

pub struct RomData {
    pub header: RomHeader,
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub chr_ram: Vec<u8>
}

impl RomData {
    pub fn new(dump: Vec<u8>) -> RomData {
        let header = RomHeader::new(dump.clone());

        let mut prg_rom = Vec::new();
        prg_rom.resize(header.prg_rom_size as usize * 0x4000, 0);

        let mut chr_rom = Vec::new();
        chr_rom.resize(header.prg_rom_size as usize * 0x2000, 0);

        let mut anchor = if header.trainer { 0x210 } else { 0x10 };

        for i in 0..prg_rom.len() {
            prg_rom[i] = dump[anchor + i];
        }

        anchor += 0x4000 * header.prg_rom_size as usize; 

        for i in 0..chr_rom.len() {
            chr_rom[i] = dump[anchor + i];
        }

        let rom_data = RomData {
            header: header,
            prg_rom: prg_rom,
            prg_ram: Vec::new(),
            chr_rom: chr_rom,
            chr_ram: Vec::new()
        };

        rom_data
    }
}