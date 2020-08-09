use std::fs;

pub struct Cartridge {
    header: [u8; 16],
    trainer: [u8; 512],
    mapper_id: u8,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>
}

impl Cartridge {
    pub fn new() -> Cartridge {
        let cartridge = Cartridge {
            header: [0; 16],
            trainer: [0; 512],
            mapper_id: 0,
            prg_rom: Vec::new(),
            chr_rom: Vec::new()
        };
        cartridge
    }

    pub fn inject(&mut self, payload_path: &str) {
        let payload = fs::read(payload_path);
        let payload = match payload {
            Ok(g) => g,
            Err(_) => panic!("Error! No file was found at location")
        };

        for i in 0..16 {
            self.header[i] = payload[i];
        }

        self.prg_rom.resize(16384 as usize * self.header[4] as usize, 0);
        self.chr_rom.resize(8192 as usize * self.header[5] as usize, 0);
        self.mapper_id = ((self.header[6] >> 4) << 4) | (self.header[5] >> 4);

        let trainer = if self.header[5] & 0x04 != 0 { true } else { false }; // haven't verified if ROMs with trainers work

        let mut anchor = if trainer { 16 + 512 } else { 16 };

        if trainer {
            for i in 0..512 {
                self.trainer[i] = payload[anchor + i];
            }
        }

        for i in 0..(16384 as usize * self.header[4] as usize) {
            self.prg_rom[i] = payload[anchor + i];
        }

        anchor = if trainer { 16 + 512 + (16384 * self.header[4] as usize)} else { 16 + (16384 * self.header[4] as usize)};

        for i in 0..(8192 * self.header[5] as usize) {
            self.chr_rom[i] = payload[anchor + i];
        }
    }

    pub fn get_info(&self) {
        println!("\n--- ROM Information ---");
        
        print!("ROM Header:\t");
        for i in 0..16 {
            print!("{} ", self.header[i]);
        }
        
        println!();
        println!("Valid ROM?\t{}", self.header[0] == 78 || self.header[1] == 69 ||self.header[2] == 83 ||self.header[3] == 26);
        println!("Mapper ID:\t{}", self.mapper_id);
        println!("PRG ROM Size:\t{}", self.header[4] as usize * 16384);
        println!("CHR ROM Size:\t{}", self.header[5] as usize * 8192);
        print!("Mirror Config:\t{}", self.header[5] & 0x01);
        if self.header[5] & 0x08 != 0 { println!(" (ignore)") } else { println!() }
        println!("Cart Battery?\t{}", self.header[5] & 0x02 != 0);
        println!("Trainer?\t{}", self.header[5] & 0x04 != 0);
    }
}