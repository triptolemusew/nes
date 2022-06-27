use super::bus::Bus;
use super::components::{cpu::Cpu, mapper, ppu::Ppu, rom::Rom};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Emulator {
    cpu: Cpu,
    ppu: Ppu,
    bus: Bus,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        let mapper = match rom.get_mapper() {
            Some(mapper) => Rc::new(RefCell::new(
                mapper::create_mapper(mapper, rom.prg_rom.clone(), rom.chr_rom.clone()).unwrap(),
            )),
            None => panic!("Couldn't find a mapper"),
        };

        self.bus.attach_mapper(mapper.clone());
        self.ppu.attach_mapper(mapper.clone());
    }

    pub fn reset(&mut self) {
        let bus = &mut self.bus;

        self.cpu.reset(bus);
        self.ppu.reset();
    }

    pub fn run(&mut self) {
        let bus = &mut self.bus;

        self.cpu.cycle(bus);
        self.ppu.cycle(bus);
    }
}
