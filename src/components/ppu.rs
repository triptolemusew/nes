use crate::bus::Bus;

pub struct Ppu {
    vram: Vec<u8>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0x00; 2048],
        }
    }

    pub fn cycle(&mut self, bus: &mut Bus) {}
}
