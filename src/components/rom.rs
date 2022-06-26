use super::mapper;
use std::{fs::*, io::Read};

#[derive(Debug)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub contents: Vec<u8>,
}

impl Rom {
    pub fn new(path: &String) -> Self {
        let mut f = File::open(&path).expect("no file found");
        let metadata = std::fs::metadata(&path).expect("unable to load metadata");

        let mut contents = vec![0; metadata.len() as usize];

        f.read_exact(&mut contents).expect("overflow");

        let prg_rom_size = contents[4] as usize;
        let chr_rom_size = contents[5] as usize;

        let skip_trainer = contents[6] & 0b100 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Rom {
            prg_rom: contents.clone()[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: contents.clone()[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            contents,
        }
    }

    pub fn get_magic_number(&self) -> String {
        self.contents[..0x4].iter().map(|x| *x as char).collect()
    }

    pub fn get_mapper(&self) -> Option<mapper::MapperType> {
        let value = ((self.contents[6] >> 4) & 0xF) | (self.contents[7] & 0xF0);
        match value {
            0 => Some(mapper::MapperType::NROM),
            _ => None,
        }
    }

    pub fn get_vbanks(&self) -> u8 {
        self.contents[5]
    }
}
