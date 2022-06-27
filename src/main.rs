#[macro_use]
extern crate lazy_static;
extern crate bitflags;

use nes::components::rom::Rom;
use nes::run;

use std::env;

pub mod bus;
pub mod components;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        0..=2 => args.get(1).unwrap(),
        _ => panic!("Please provide rom."),
    };

    let rom = Rom::new(&String::from(file_name));

    pollster::block_on(run(&rom));
}
