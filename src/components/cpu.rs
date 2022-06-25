use crate::bus::Bus;

#[allow(clippy::upper_case_acronyms)]
pub enum InterruptVector {
    NMI = 0xFFFA,
    Reset = 0xFFFC,
    IRQ = 0xFFFE,
}

#[derive(Default)]
pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    x: u8,
    y: u8,
    cycles: usize,
    ram: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            ram: vec![0x00; 0x800],
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.pc = 0;
        // self.pc = InterruptVector::ResetVector as u16;
    }

    pub fn get_word(&mut self, bus: &Bus) -> u16 {
        let hi = bus.read_memory(self.pc);
        let lo = bus.read_memory(self.pc + 1);
        u16::from_be_bytes([hi, lo])
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        self.cycles += 1;

        let opcode = bus.read_memory(self.pc);

        println!("pc: {:#X} opcode: {:#X}", self.pc, opcode);
    }
}
