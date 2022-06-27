use std::collections::HashMap;

pub enum Mode {
    Implied,
    Absolute,
}

pub struct Opcode {
    pub code: u8,
    pub mode: Mode,
}

lazy_static::lazy_static! {
    pub static ref OPCODES: Vec<Opcode> = vec![
        Opcode {code: 0x78, mode: Mode::Implied},
    ];
    pub static ref OPCODE_MAP: HashMap<u8, &'static Opcode> = {
        let mut map = HashMap::new();
        for cpuop in &*OPCODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
