pub struct Bus {
    ram: Vec<u8>,
}

/*
   CPU Memory Map
   $0000 - $07FF -> 2KB Internal Ram
   $2000 - $2007 -> PPU Registers
   $4000 - $4017 -> APU & I/O registers
   $4018 - $401F -> APU
   $4020 - $FFFF -> Cartridge space: PRG ROM, PRG RAM, and Mapper registers
*/

impl<'a> Bus {
    pub fn new() -> Self {
        Bus {
            ram: vec![0x00; 0x800],
        }
    }

    pub fn read(&self, mut addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
}
