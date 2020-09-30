pub struct RomHeader {
    pub valid: bool,        // bytes 0-3
    pub prg_rom_size: u8,   // PRG ROM pages (16 KB units)
    pub chr_rom_size: u8,   // CHR ROM pages (8 KB units, 0 means the board uses CHR RAM)
    pub mapper_id: u8,      
    pub mirror_mode: u8,
    pub mirror_ignore: bool,
    pub battery: bool,
    pub trainer: bool
}

impl RomHeader {
    pub fn new(dump: Vec<u8>) -> RomHeader {
        let rom_header = RomHeader {
            valid: if dump[0] == 0x4E && dump[1] == 0x45 && dump[2] == 0x53 && dump[3] == 0x1A { true } else { false }, 
            prg_rom_size: dump[4],
            chr_rom_size: dump[5],
            mapper_id: (dump[6] >> 4) | (dump[7] & 0xF0),
            mirror_mode: if dump[6] & 1 != 0 { 1 } else { 0 },
            battery: dump[6] & 0x02 != 0,
            trainer: dump[6] & 0x04 != 0,
            mirror_ignore: dump[6] & 0x08 != 0
        };

        print!("Header:\t\t");
        for i in 0..15 { print!("{} ", dump[i]); }
        println!("");
        println!("Valid:\t\t{}", rom_header.valid);
        println!("PRG ROM Pages:\t{}", rom_header.prg_rom_size);
        println!("CHR ROM Pages:\t{}", rom_header.chr_rom_size);
        println!("Mapper ID:\t{}", rom_header.mapper_id);
        println!("Mirror Mode:\t{}", rom_header.mirror_mode);
        println!("Battery:\t{}", rom_header.battery);
        println!("Trainer:\t{}", rom_header.trainer);
        println!("Ignore Mirror:\t{}", rom_header.mirror_ignore);

        rom_header
    }
}