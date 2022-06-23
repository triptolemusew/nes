use std::{fs::*, io::Read};

#[derive(Debug)]
pub struct Rom {
    pub contents: Vec<u8>,
}

impl Rom {
    pub fn new(path: &String) -> Self {
        let mut f = File::open(&path).expect("no file found");
        let metadata = std::fs::metadata(&path).expect("unable to load metadata");

        let mut contents = vec![0; metadata.len() as usize];

        f.read_exact(&mut contents).expect("overflow");

        Rom { contents }
    }

    pub fn get_magic_number(&self) -> String {
        self.contents[..0x4].iter().map(|x| *x as char).collect()
    }

    pub fn get_mapper_number(&self) -> u8 {
        ((self.contents[6] >> 4) & 0xF) | (self.contents[7] & 0xF0)
    }

    pub fn get_vbanks(&self) -> u8 {
        self.contents[5]
    }
}
