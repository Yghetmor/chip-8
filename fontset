use std::fs::{File, read};
use rand::prelude::*;

const START_ADDRESS: u32 = 0x200;
const FONTSET_SIZE: u32 = 80;
const FONTSET_START_ADDRESS: u32 = 0x50;
const VIDEO_WIDTH: u32 = 680;
const VIDEO_HEIGHT: u32 = 320;

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
        let mut _i = 0;
        for byte in buf {
            self.memory[START_ADDRESS as usize] = byte;
            _i += 1;
        }
    }

    pub fn OP_00E0(&mut self) {
        for elem in self.video.iter_mut() {
           *elem = 0;
        }
    }

    pub fn OP_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    pub fn OP_1nnn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.pc = address;
    }

    pub fn OP_2nnn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }   

    pub fn OP_3xkk(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let byte: u8 = (self.opcode & 0x00FF).try_into().unwrap();

        if self.registers[Vx as usize] == byte {
            self.pc += 2;
        }
    }

    pub fn OP_4xkk(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let byte: u8 = (self.opcode & 0x00FF).try_into().unwrap();

        if self.registers[Vx as usize] != byte {
            self.pc += 2;
        }
    }

    pub fn OP_5xy0(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        if self.registers[Vx as usize] == self.registers[Vy as usize] {
            self.pc += 2;
        }
    }

    pub fn OP_6xkk(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let byte: u8 = (self.opcode & 0x00FF).try_into().unwrap();

        self.registers[Vx as usize] = byte;
    }

    pub fn OP_7xkk(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let byte: u8 = (self.opcode & 0x00FF).try_into().unwrap();

        self.registers[Vx as usize] += byte;
    }

    pub fn OP_8xy0(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        self.registers[Vx as usize] = self.registers[Vy as usize];
    }

    pub fn OP_8xy1(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        self.registers[Vx as usize] |= self.registers[Vy as usize];
    }

    pub fn OP_8xy2(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        self.registers[Vx as usize] &= self.registers[Vy as usize];
    }

    pub fn OP_8xy3(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        self.registers[Vx as usize] ^= self.registers[Vy as usize];
    }

    pub fn OP_8xy4(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        let sum: u16 = self.registers[Vx as usize] as u16 + self.registers[Vy as usize] as u16;

        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[Vx as usize] = (sum & 0xFF).try_into().unwrap();
    }

    pub fn OP_8xy5(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        if self.registers[Vx as usize] > self.registers[Vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[Vx as usize] -= self.registers[Vy as usize];
    }

    pub fn OP_8xy6(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        self.registers[0xF] = self.registers[Vx as usize] & 0x1;

        self.registers[Vx as usize] >>= 1;
    }

    pub fn OP_8xy7(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        if self.registers[Vy as usize] > self.registers[Vx as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[Vx as usize] = self.registers[Vy as usize] - self.registers[Vx as usize];
    }

    pub fn OP_8xyE(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        self.registers[0xF] = (self.registers[Vx as usize] & 0x80) >> 7;

        self.registers[Vx as usize] <<= 1;
    }

    pub fn OP_9xy0(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();

        if self.registers[Vy as usize] != self.registers[Vx as usize] {
            self.pc += 2;
        }
    }

    pub fn OP_Annn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;

        self.index = address;
    }

    pub fn OP_Bnnn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;

        self.pc = self.registers[0] as u16 + address;
    }

    pub fn OP_Cxkk(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let byte: u8 = (self.opcode & 0x00FF).try_into().unwrap();

        self.registers[Vx as usize] = rand::random::<u8>() & byte;
    }

    pub fn OP_Dxyn(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let Vy: u8 = ((self.opcode & 0x00F0) >> 4).try_into().unwrap();
        let height: u8 = (self.opcode & 0x000F).try_into().unwrap();

        let xPos: u8 = self.registers[Vx as usize].try_into().unwrap() % VIDEO_WIDTH;
        let yPos: u8 = self.registers[Vy as usize].try_into().unwrap() % VIDEO_HEIGHT;

        self.registers[0xF] = 0;

        for row in [0..height] {
            let spriteByte: u8 = self.memory[self.index as usize + row as usize];
        }
    }
}

#[cfg(test)]
mod tests {

}
