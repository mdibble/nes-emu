// NES emulator written in Rust
// Developed by Matthew Dibble
// Started August 9th, 2020

// "For young players, classic games are brand new." 
//  -Satoru Iwata

extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;

mod cartridge;
mod nes;
mod cpu;
mod bus;
mod ppu;
mod joypad;

use nes::NES;

use std::fs;

fn main() {
    let cart_data = fs::read("roms/donkey_kong.nes");

    let cart_data = match cart_data {
        Ok(g) => g,
        Err(_) => panic!("Error! No file was found at the location specified")
    };

    let mut nes = NES::new(cart_data);

    nes.reset();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("NES Emulator", 256 * 2, 240 * 2).position_centered().build().unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 256, 240).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        nes.cycle();
        if nes.cpu.bus.ppu.draw {
            nes.draw(&mut canvas, &mut texture);
            let mut inputs = nes.cpu.bus.joypad.read();
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::X), .. } => inputs |= 0x80,
                    Event::KeyDown { keycode: Some(Keycode::Z), .. } => inputs |= 0x40,
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => inputs |= 0x20,
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => inputs |= 0x10,
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => inputs |= 0x08,
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => inputs |= 0x04,
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => inputs |= 0x02,
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => inputs |= 0x01,
                    Event::KeyUp { keycode: Some(Keycode::X), .. } => inputs &= !0x80,
                    Event::KeyUp { keycode: Some(Keycode::Z), .. } => inputs &= !0x40,
                    Event::KeyUp { keycode: Some(Keycode::A), .. } => inputs &= !0x20,
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => inputs &= !0x10,
                    Event::KeyUp { keycode: Some(Keycode::Up), .. } => inputs &= !0x08,
                    Event::KeyUp { keycode: Some(Keycode::Down), .. } => inputs &= !0x04,
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => inputs &= !0x02,
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => inputs &= !0x01,
                    _ => {}
                }
            }
            nes.cpu.bus.joypad.write(inputs)
        }
    }
}