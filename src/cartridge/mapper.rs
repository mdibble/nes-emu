pub trait Mapper {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, contents: u8) -> u8;
}