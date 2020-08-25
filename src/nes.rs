use crate::cpu::CPU;

use std::fs;
use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct NES {
    cpu: CPU
}

impl NES {
    pub fn new(cart_data: Vec<u8>) -> NES {
        let nes = NES {
            cpu: CPU::new(cart_data)
        };
        nes
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn cycle(&mut self) {
        self.cpu.tick();
        self.cpu.bus.ppu.tick();
        self.cpu.bus.ppu.tick();
        self.cpu.bus.ppu.tick();
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        for row in 0..240 {
            for col in 0..256 {
                canvas.set_draw_color(Color::RGB(self.cpu.bus.ppu.display[row * 240 + row].r, self.cpu.bus.ppu.display[row * 240 + row].g, self.cpu.bus.ppu.display[row * 240 + row].b));
                canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
            }
        }
        canvas.present();
    }
}