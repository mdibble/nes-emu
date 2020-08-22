// NES Emulator written in Rust
// Developed by Matthew Dibble
// Started August 9th, 2020

extern crate sdl2;

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

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem.window("NES Emulator", 512, 480).position_centered().build().unwrap();
    
    // let mut canvas = window.into_canvas().build().unwrap();
    // canvas.clear();
    // canvas.present();

    // nes.draw(&mut canvas);

    loop {
        nes.cycle();
    }
}