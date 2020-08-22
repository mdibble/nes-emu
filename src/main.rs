// NES Emulator written in Rust
// Developed by Matthew Dibble
// Started August 9th, 2020

mod cartridge;
mod nes;
mod cpu;
mod bus;
mod ppu;

use nes::NES;

use std::fs;

fn main() {
    let cart_data = fs::read("roms/donkey_kong.nes");

    let cart_data = match cart_data {
        Ok(g) => g,
        Err(_) => panic!("Error! No file was found at the location specified")
    };

    let mut nes = NES::new(cart_data);
    // For testing the CPU without graphics
    // Tested until: $c6bd, cycle 14579
    // nes.cpu_test("roms/nestest.nes");

    nes.reset();

    loop {
        nes.cycle();
    }
}