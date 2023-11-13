mod chip;
mod platform;

use crate::chip::Chip8;
use crate::platform::Platform;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Too many or too few arguments");
        process::exit(1);
    }
    let rom_path = &args[1];
    
    let mut chip = Chip8::new();
    //chip.load_rom(rom_path);

    let mut plat = Platform::new();

    while !plat.quit {
        plat.update();
        plat.process_input(&mut chip.keypad);
    }
}
