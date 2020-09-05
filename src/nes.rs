use crate::cpu::CPU;

use sdl2::render::WindowCanvas;
use sdl2::render::Texture;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

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

    pub fn draw(&mut self, canvas: &mut WindowCanvas, texture: &mut Texture) {
        texture.update(None, &self.cpu.bus.ppu.display, 256 * 3 as usize).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        // for row in 0..30 {
        //     for col in 0..32 {
        //         let val = self.cpu.bus.ppu.get_memory(0x2000 + (row * 32) + col);
        //         canvas.set_draw_color(Color::RGB(val, val, val));
        //         canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
        //     }
        // }

        // for row in 0..30 {
        //     for col in 32..64 {
        //         let val = self.cpu.bus.ppu.get_memory(0x2400 + (row * 32) + (col - 32));
        //         canvas.set_draw_color(Color::RGB(val, val, val));
        //         canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
        //     }
        // }

        canvas.present();
        self.cpu.bus.ppu.draw = false;
    }
}