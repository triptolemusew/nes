use super::rom::Rom;

#[allow(clippy::upper_case_acronyms)]
pub enum MapperType {
    NROM,
    SxROM,
    UxROM,
}

pub trait Mapper {
    fn read_prg(&self, addr: u16) -> u8;
    fn write_prg(&mut self, addr: u16, value: u8);

    fn read_chr(&self, addr: u16) -> u8;
    fn write_chr(&mut self, addr: u16, value: u8);
}

pub struct MapperNRom<'a> {
    rom: &'a Rom,
}

#[allow(unused_variables)]
impl<'a> Mapper for MapperNRom<'a> {
    fn read_prg(&self, addr: u16) -> u8 {
        self.rom.contents[addr as usize - 0x8000]
    }
    fn read_chr(&self, addr: u16) -> u8 {
        self.rom.contents[addr as usize]
    }
    fn write_chr(&mut self, addr: u16, value: u8) {}
    fn write_prg(&mut self, addr: u16, value: u8) {}
}

impl<'a> MapperNRom<'a> {
    pub fn new(rom: &'a Rom) -> Self {
        MapperNRom { rom }
    }
}

pub fn create_mapper<'a>(mapper: MapperType, rom: &'a Rom) -> Option<Box<dyn Mapper + 'a>> {
    match mapper {
        NROM => Some(Box::new(MapperNRom::new(rom))),
        _ => unimplemented!(),
    }
}
