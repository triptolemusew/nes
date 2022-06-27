use crate::bus::Bus;
use crate::components::mapper;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Ppu {
    ctrl: u8,
    mask: u8,
    status: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    oam_addr: u8,
    oam_data: u8,
    oam_dma: u8,
    vram: Vec<u8>,
    sprite: Vec<u8>,
    mapper: Option<Rc<RefCell<Box<dyn mapper::Mapper>>>>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            sprite: vec![0x00; 64 * 4],
            vram: vec![0x00; 256 * 240],
            ..Default::default()
        }
    }

    pub fn attach_mapper(&mut self, mapper: Rc<RefCell<Box<dyn mapper::Mapper>>>) {
        self.mapper = Some(mapper);
    }

    pub fn reset(&mut self) {}

    pub fn cycle(&mut self, bus: &mut Bus) {}
}
