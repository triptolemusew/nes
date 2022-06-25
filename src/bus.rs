pub struct Bus {
    ram: Vec<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: vec![0x00; 0xFFFF],
        }
    }

    pub fn read_memory(&self, mut addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_memory(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
}
