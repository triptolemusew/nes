use std::{fs::*, io::Read};

use super::mapper;

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

        f.read_exact(&mut contents).unwrap();

        let banks = contents[4] as usize;
        let vbanks = contents[5] as usize;

        let prg_rom_size = 0x4000 * banks;
        let chr_rom_size = 0x2000 * vbanks;

        let prg_rom_start = 0x10;
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let prg_rom = contents[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec();
        let chr_rom = contents[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec();

        // for (i, item) in prg_rom.iter().enumerate().step_by(16) {
        //     print!("{:#X}: ", i);
        //     for x in prg_rom[i..(i + 16)].iter() {
        //         print!("{:#X} ", x);
        //     }
        //     println!();
        // }

        Rom {
            prg_rom,
            chr_rom,
            contents,
        }
    }

    pub fn get_magic_number(&self) -> String {
        self.contents[..0x4].iter().map(|x| *x as char).collect()
    }

    pub fn get_mapper(&self) -> Option<mapper::MapperType> {
        use mapper::MapperType::*;

        let value = ((self.contents[6] >> 4) & 0xF) | (self.contents[7] & 0xF0);
        match value {
            0 => Some(NROM),
            _ => None,
        }
    }

    pub fn get_vbanks(&self) -> u8 {
        self.contents[5]
    }
}
