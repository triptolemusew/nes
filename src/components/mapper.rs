use std::fmt::Debug;

#[allow(clippy::upper_case_acronyms)]
pub enum MapperType {
    NROM,
    UxROM,
}

pub trait Mapper {
    fn read_prg(&self, addr: u16) -> u8;
    fn write_prg(&mut self, addr: u16, value: u8);
    fn read_chr(&self, addr: u16) -> u8;
    fn write_chr(&mut self, addr: u16, value: u8);
}

pub struct MapperNRom {
    prg: Vec<u8>,
    chr: Vec<u8>,
}

impl Debug for dyn Mapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mapper()")
    }
}

impl Mapper for MapperNRom {
    fn read_prg(&self, addr: u16) -> u8 {
        self.prg[addr as usize - 0x8000]
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        self.prg[addr as usize - 0x8000] = value;
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr[addr as usize]
    }

    fn write_chr(&mut self, addr: u16, value: u8) {
        self.chr[addr as usize] = value;
    }
}

impl MapperNRom {
    pub fn new(prg: Vec<u8>, chr: Vec<u8>) -> Self {
        MapperNRom { prg, chr }
    }
}

pub fn create_mapper(mapper: MapperType, prg: Vec<u8>, chr: Vec<u8>) -> Option<Box<dyn Mapper>> {
    match mapper {
        NROM => Some(Box::new(MapperNRom::new(prg, chr))),
        _ => unimplemented!(),
    }
}
