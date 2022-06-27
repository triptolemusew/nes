use std::collections::HashMap;

pub enum Mode {
    Implied,
    Absolute,
    Immediate,
    Relative,
}

pub struct Opcode {
    pub code: u8,
    pub mode: Mode,
}

lazy_static::lazy_static! {
    pub static ref OPCODES: Vec<Opcode> = vec![
        Opcode {code: 0x10, mode: Mode::Relative},
        Opcode {code: 0x78, mode: Mode::Implied},
        Opcode {code: 0x8D, mode: Mode::Absolute},
        Opcode {code: 0x9A, mode: Mode::Implied},
        Opcode {code: 0xA2, mode: Mode::Immediate},
        Opcode {code: 0xA9, mode: Mode::Immediate},
        Opcode {code: 0xAD, mode: Mode::Absolute},
        Opcode {code: 0xD8, mode: Mode::Implied},
    ];

    pub static ref OPCODE_MAP: HashMap<u8, &'static Opcode> = {
        let mut map = HashMap::new();
        for cpuop in &*OPCODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
