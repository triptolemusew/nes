use std::env;

mod bus;
mod cpu;
mod rom;

use crate::rom::Rom;
use nes::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        _ => args.get(1).unwrap(),
    };

    let rom = Rom::new(&String::from(file_name));

    println!("{:?}", &rom.contents[..0x20]);

    pollster::block_on(run());
}
