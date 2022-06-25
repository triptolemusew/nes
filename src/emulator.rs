use super::bus::Bus;
use super::components::{cpu::Cpu, mapper, ppu::Ppu};
use super::rom::Rom;

pub struct Emulator<'a> {
    cpu: Cpu,
    ppu: Ppu,
    bus: Bus,
    mapper: Option<Box<dyn mapper::Mapper + 'a>>,
}

impl<'a> Emulator<'a> {
    pub fn new() -> Self {
        Emulator {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            bus: Bus::new(),
            mapper: None,
        }
    }

    pub fn load_rom(&mut self, rom: &'a Rom) {
        self.mapper = match rom.get_mapper() {
            Some(mapper) => mapper::create_mapper(mapper, rom),
            None => panic!("Couldn't find a mapper"),
        };

        for (i, item) in rom.contents.iter().enumerate() {
            self.bus.write_memory(i as u16, *item);
        }
    }

    pub fn run(&mut self) {
        let bus = &mut self.bus;

        self.cpu.cycle(bus);
        self.ppu.cycle(bus);
    }
}
