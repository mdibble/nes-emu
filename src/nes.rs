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
                canvas.set_draw_color(Color::RGB(self.cpu.bus.ppu.display[row * 256 + col].r, self.cpu.bus.ppu.display[row * 256 + col].g, self.cpu.bus.ppu.display[row * 256 + col].b));
                canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
            }
        }

        let mut pattern_table_0: Vec<u8> = Vec::new();
        let mut pattern_table_1: Vec<u8> = Vec::new();
        pattern_table_0.resize(128 * 128, 0);
        pattern_table_1.resize(128 * 128, 0);

        for tile_y in 0..16 {
            for tile_x in 0..16 {
                //println!("Tile: ${:04x} (${:04x})", (tile_y * 256) + (tile_x * 16), (tile_y * 256) + (tile_x + 16));
                let offset = (tile_y * 256) + (tile_x * 16);

                for row in 0..8 {
                    let tile_lsb = self.cpu.bus.ppu.get_memory(offset + row + 0x0000);
                    let tile_msb = self.cpu.bus.ppu.get_memory(offset + row + 0x0008);

                    //println!("\tLo: {:02x} ({:08b}) Hi: {:02x} ({:08b})", tile_lsb, tile_lsb, tile_msb, tile_msb);
                    for col in 0..8 {
                        let pixel = (((tile_msb >> (7 - col)) & 0x01) << 1) | ((tile_lsb >> (7 - col)) & 0x01);
                        //print!("{}\t", pixel);

                        let mut index = (8 * 16 * (8 * tile_y)) + (tile_x * 8); // base tile
                        index += col + (row * 128);

                        //println!("\t\tDone tile ({}, {}) pixel ({}, {}) - index: {}", tile_x, tile_y, col, row, index);
                        pattern_table_0[index as usize] = pixel;
                    }
                }
            }
        }

        for tile_y in 0..16 {
            for tile_x in 0..16 {
                //println!("Tile: ${:04x} (${:04x})", (tile_y * 256) + (tile_x * 16), (tile_y * 256) + (tile_x + 16));
                let offset = (tile_y * 256) + (tile_x * 16);

                for row in 0..8 {
                    let tile_lsb = self.cpu.bus.ppu.get_memory(0x1000 + offset + row + 0x0000);
                    let tile_msb = self.cpu.bus.ppu.get_memory(0x1000 + offset + row + 0x0008);

                    //println!("\tLo: {:02x} ({:08b}) Hi: {:02x} ({:08b})", tile_lsb, tile_lsb, tile_msb, tile_msb);
                    for col in 0..8 {
                        let pixel = (((tile_msb >> (7 - col)) & 0x01) << 1) | ((tile_lsb >> (7 - col)) & 0x01);
                        //print!("{}\t", pixel);

                        let mut index = (8 * 16 * (8 * tile_y)) + (tile_x * 8); // base tile
                        index += col + (row * 128);

                        //println!("\t\tDone tile ({}, {}) pixel ({}, {}) - index: {}", tile_x, tile_y, col, row, index);
                        pattern_table_1[index as usize] = pixel;
                    }
                }
            }
        }

        for row in 0..128 {
            for col in 256..(128 * 3) {
                canvas.set_draw_color(Color::RGB(pattern_table_0[row * 128 + (col - 256)] * 0x40, pattern_table_0[row * 128 + (col - 256)] * 0x40, pattern_table_0[row * 128 + (col - 256)] * 0x40));
                canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
            }
        }

        for row in 0..128 {
            for col in 384..128 * 4 {
                canvas.set_draw_color(Color::RGB(pattern_table_1[row * 128 + (col - 384)] * 0x40, pattern_table_1[row * 128 + (col - 384)] * 0x40, pattern_table_1[row * 128 + (col - 384)] * 0x40));
                canvas.fill_rect(Rect::new(col as i32 * 2, row as i32 * 2, 2, 2)).unwrap();
            }
        }

        canvas.present();
        self.cpu.bus.ppu.draw = false;
    }
}