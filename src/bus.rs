use super::components::mapper;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Bus {
    ram: Vec<u8>,
    mapper: Option<Rc<RefCell<Box<dyn mapper::Mapper>>>>,
}

/*
   CPU Memory Map
   $0000 - $07FF -> 2KB Internal Ram
   $2000 - $2007 -> PPU Registers
   $4000 - $4017 -> APU & I/O registers
   $4018 - $401F -> APU
   $4020 - $FFFF -> Cartridge space: PRG ROM, PRG RAM, and Mapper registers
*/

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: vec![0x00; 0x800],
            ..Default::default()
        }
    }

    pub fn attach_mapper(&mut self, mapper: Rc<RefCell<Box<dyn mapper::Mapper>>>) {
        self.mapper = Some(mapper)
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapper = self.mapper.as_ref().unwrap();

        match addr {
            0x0000..=0x1FFF => self.ram[addr as usize & 0x7FF],
            0x2000..=0x4019 => {
                // PPU registers
                if addr < 0x4000 {
                    return 1;
                }
                return 2;
            },
            0x4020..=0x5FFF => {
                unimplemented!("Exapnsion ROM")
            }
            0x6000..=0x7FFF => {
                unimplemented!("Extended RAM")
            }
            _ => {
                mapper.borrow().read_prg(addr)
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let mapper = self.mapper.as_ref().unwrap();

        match addr {
            0x0000..=0x1FFF => {}
            0x2000..=0x3FFF => {}
            0x4000..=0x4013 => {
                // APU
            }
            0x4014 => {
                // OAM DMA
            }
            0x4016..=0x4017 => {
                // Controller
            }
            _ => unimplemented!(),
        }
        self.ram[addr as usize] = value;
    }
}
