use crate::cartridge::Cartridge;

pub struct NES {
    cart: Cartridge
}

impl NES {
    pub fn new() -> NES {
        let nes = NES {
            cart: Cartridge::new()
        };
        nes
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cart = cartridge;
        self.cart.get_info();
    }
}