use crate::bus::Bus;

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
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.pc = 0;
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        self.cycles += 1;

        let opcode = bus.read_memory(self.pc);

        println!("opcode: {:#X}", opcode);

        // NOTE: Execute the instruction
    }
}
