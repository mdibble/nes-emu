// NES Emulator written in Rust
// Developed by Matthew Dibble
// Started August 9th, 2020

mod cartridge;
mod nes;

use cartridge::Cartridge;
use nes::NES;

fn main() {
    let mut rom = Cartridge::new();
    rom.inject("roms/donkey_kong.nes");

    let mut nes = NES::new();
    nes.insert_cartridge(rom);
}