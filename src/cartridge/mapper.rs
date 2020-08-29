pub trait Mapper {
    fn prg_read(&self, address: u16) -> u8;
    fn prg_write(&mut self, address: u16, contents: u8) -> u8;
    fn chr_read(&self, address: u16) -> u8;
    fn chr_write(&mut self, address: u16, contents: u8) -> u8;
}