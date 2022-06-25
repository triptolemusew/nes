use crate::bus::Bus;

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
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0x00; 2048],
            ..Default::default()
        }
    }

    pub fn cycle(&mut self, bus: &mut Bus) {}
}
