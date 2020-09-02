use crate::cpu::CPU;

use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct NES {
    pub cpu: CPU
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
                canvas.set_draw_color(Color::RGB(self.cpu.bus.ppu.display[(row * 256) + col].r, self.cpu.bus.ppu.display[(row * 256) + col].g, self.cpu.bus.ppu.display[(row * 256) + col].b));
                canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
            }
        }

        for row in 128..128 + 30 {
            for col in 256..256 + 32 {
                let val = self.cpu.bus.ppu.get_memory(0x2000 + ((row - 128) * 32) + (col - 256));
                canvas.set_draw_color(Color::RGB(val, val, val));
                canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
            }
        }
        canvas.present();
        self.cpu.bus.ppu.draw = false;
    }
}