use std::fs::{File, read};
use rand::prelude::*;

const START_ADDRESS: u32 = 0x200;
const FONTSET_SIZE: u32 = 80;
const FONTSET_START_ADDRESS: u32 = 0x50;

const fontset: [u8; FONTSET_SIZE as usize] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    video: [u32; 64 * 32],
    opcode: u16,
    randByte: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip = Chip8 {
            registers : [0; 16],
            memory : [0; 4096],
            index : 0,
            pc : START_ADDRESS as u16,
            stack : [0; 16],
            sp : 0,
            delay_timer : 0,
            sound_timer : 0,
            keypad : [0; 16],
            video : [0; 64 * 32],
            opcode : 0,
            randByte : 0,
        };

        let mut i = 0;
        for elem in fontset {
            chip.memory[(FONTSET_START_ADDRESS + i) as usize] = elem;
            i += 1;
        }

        chip.randByte = random();

        chip
    }

    pub fn LoadROM(&mut self, filename: String) {
        let buf = read(filename).unwrap();
        let mut i = 0;
        for byte in buf {
            self.memory[START_ADDRESS as usize] = byte;
            i += 1;
        }
    }

    pub fn OP_00E0(&mut self) {
        
    }
}

#[cfg(test)]
mod tests {

}