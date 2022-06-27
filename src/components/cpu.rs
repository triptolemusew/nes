use crate::bus::Bus;
use crate::components::opcode::{Mode, Opcode, OPCODE_MAP};

bitflags::bitflags! {
    #[derive(Default)]
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

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
    status: CpuFlags,
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

    fn get_word(&mut self, bus: &Bus) -> u16 {
        let hi = bus.read(self.pc) as u16;
        let lo = bus.read(self.pc + 1) as u16;
        hi | (lo << 8)
    }

    fn get_byte(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        let op = bus.read(self.pc);
        self.pc += 1;

        let opcode = OPCODE_MAP
            .get(&op)
            .expect(format!("Opcode is not recognized: {:#X}", op).as_str());

        let cycle = match opcode.mode {
            Mode::Implied => self.do_implied(opcode, bus),
            Mode::Immediate => self.do_immediate(opcode, bus),
            Mode::Absolute => self.do_absolute(opcode, bus),
            _ => panic!("Unimplemented opcode: {:#X}", opcode.code),
        };

        println!("Opcode: {:#X}, ", opcode.code);

        self.cycles += cycle as usize;
    }

    fn do_implied(&mut self, opcode: &Opcode, bus: &mut Bus) -> u8 {
        match opcode.code {
            0x78 => {
                self.status.insert(CpuFlags::INTERRUPT_DISABLE);
                2
            }
            0xD8 => {
                self.status.remove(CpuFlags::DECIMAL_MODE);
                2
            }
            _ => unimplemented!(),
        }
    }

    fn do_immediate(&mut self, opcode: &Opcode, bus: &mut Bus) -> u8 {
        match opcode.code {
            0xA9 => {
                self.a = self.get_byte(bus);
                self.set_zn(self.a);
                2
            }
            _ => unimplemented!(),
        }
    }

    fn do_absolute(&mut self, opcode: &Opcode, bus: &mut Bus) -> u8 {
        match opcode.code {
            // STA
            0x8D => {
                let addr = self.get_word(bus);
                bus.write(addr, self.a);
                4
            }
            _ => unimplemented!()
        }
    }

    fn set_zn(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }

        if value >> 7 == 1 {
            self.status.insert(CpuFlags::NEGATIVE);
        } else {
            self.status.remove(CpuFlags::NEGATIVE);
        }
    }
}
