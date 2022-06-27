use crate::bus::Bus;
use crate::components::opcode::{OPCODE_MAP, Mode, Opcode};

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
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            ..Default::default()
        }
    }

    pub fn reset(&mut self, bus: &Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;

        let lo = bus.read(InterruptVector::Reset as u16) as u16;
        let hi = bus.read(InterruptVector::Reset as u16 + 1) as u16;
        self.pc = (hi << 8) | lo;
    }

    pub fn get_word(&mut self, bus: &Bus) -> u16 {
        let hi = bus.read(self.pc);
        let lo = bus.read(self.pc + 1);
        u16::from_be_bytes([hi, lo])
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        let op = bus.read(self.pc);

        let opcode = OPCODE_MAP
            .get(&op)
            .expect(format!("Opcode is not recognized: {:#X}", op).as_str());

        let cycle = match opcode.mode {
            Mode::Implied => self.do_implied(opcode, bus),
            _ => panic!("Unimplemented opcode: {:#X}", opcode.code),
        };

        self.pc += 1;
        self.cycles += cycle as usize;
    }

    fn do_implied(&mut self, opcode: &Opcode, bus: &mut Bus) -> u8 {
        match opcode.code {
            // SEI
            0x78 => {
                2
            }
            _ => panic!("Unimplemented")
        }
    }
}
