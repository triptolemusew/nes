use nes::run;
use nes::components::rom::Rom;

use std::env;

mod components;
mod bus;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        0..=2 => args.get(1).unwrap(),
        _ => panic!("Please provide rom."),
    };

    let rom = Rom::new(&String::from(file_name));

    pollster::block_on(run(&rom));
}
