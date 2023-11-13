use std::fs::read;
use std::process;
use rand::prelude::*;

const START_ADDRESS: u32 = 0x200;
const FONTSET_SIZE: u32 = 80;
const FONTSET_START_ADDRESS: u8 = 0x50;
const VIDEO_WIDTH: u32 = 680;
const VIDEO_HEIGHT: u32 = 320;

const FONTSET: [u8; FONTSET_SIZE as usize] = [
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
    pub keypad: [u8; 16],
    video: [u32; 64 * 32],
    opcode: u16,
    rand_byte: u8,
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
            rand_byte : 0,
        };

        let mut i = 0;
        for elem in FONTSET {
            chip.memory[(FONTSET_START_ADDRESS + i) as usize] = elem;
            i += 1;
        }

        chip.rand_byte = random();

        chip
    }

    pub fn load_rom(&mut self, filename: String) {
        let buf = read(filename).unwrap_or_else(|err| {
            eprintln!("Could not open ROM: {err}");
            process::exit(1);
        });
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

        let xPos: u8 = (self.registers[Vx as usize] as u32 % VIDEO_WIDTH).try_into().unwrap();
        let yPos: u8 = (self.registers[Vy as usize] as u32 % VIDEO_HEIGHT).try_into().unwrap();

        self.registers[0xF] = 0;
        
        let row = 0;
        
        while row < height {
            let sprite_byte: u8 = self.memory[self.index as usize + row as usize];
            let col = 0;

            while col < 8 {
                let sprite_pixel: u8 = sprite_byte & (0x80 >> col);
                let mut screen_pixel = self.video[(yPos + row) as usize * VIDEO_WIDTH as usize + (xPos + col) as usize];

                if sprite_pixel != 0 {
                    if screen_pixel == 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }

                    screen_pixel ^= 0xFFFFFFFF;
                }
            }
        }
    }

    pub fn OP_Ex9E(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let key: u8 = self.registers[Vx as usize];

        if self.keypad[key as usize] != 0 {
            self.pc += 2;
        }
    }

    pub fn OP_ExA1(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let key: u8 = self.registers[Vx as usize];

        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }

    pub fn OP_Fx07(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        self.registers[Vx as usize] = self.delay_timer;
    }

    pub fn OP_Fx0A(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        if self.keypad[0] != 0 {
            self.registers[Vx as usize] = 0;
        } else if self.keypad[1] != 0 {
            self.registers[Vx as usize] = 1;
        } else if self.keypad[2] != 0 {
            self.registers[Vx as usize] = 2;
        } else if self.keypad[3] != 0 {
            self.registers[Vx as usize] = 3;
        } else if self.keypad[4] != 0 {
            self.registers[Vx as usize] = 4;
        } else if self.keypad[5] != 0 {
            self.registers[Vx as usize] = 5;
        } else if self.keypad[6] != 0 {
            self.registers[Vx as usize] = 6;
        } else if self.keypad[7] != 0 {
            self.registers[Vx as usize] = 7;
        } else if self.keypad[8] != 0 {
            self.registers[Vx as usize] = 8;
        } else if self.keypad[9] != 0 {
            self.registers[Vx as usize] = 9;
        } else if self.keypad[10] != 0 {
            self.registers[Vx as usize] = 10;
        } else if self.keypad[11] != 0 {
            self.registers[Vx as usize] = 11;
        } else if self.keypad[12] != 0 {
            self.registers[Vx as usize] = 12;
        } else if self.keypad[13] != 0 {
            self.registers[Vx as usize] = 13;
        } else if self.keypad[14] != 0 {
            self.registers[Vx as usize] = 14;
        } else if self.keypad[15] != 0 {
            self.registers[Vx as usize] = 15;
        } else {
            self.pc -= 2;
        }
    }

    pub fn OP_Fx15(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        self.delay_timer = self.registers[Vx as usize];
    }

    pub fn OP_Fx18(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        self.sound_timer = self.registers[Vx as usize];
    }

    pub fn OP_Fx1E(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        self.index += self.registers[Vx as usize] as u16;
    }

    pub fn OP_Fx29(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let digit: u8 = self.registers[Vx as usize];

        self.index = (FONTSET_START_ADDRESS + (5 * digit)) as u16;
    }

    pub fn OP_Fx33(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();
        let mut value: u8 = self.registers[Vx as usize];

        self.memory[self.index as usize + 2] = value % 10;
        value /= 10;

        self.memory[self.index as usize + 1] = value % 10;
        value /=10;

        self.memory[self.index as usize] = value % 10;
    }

    pub fn OP_Fx55(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        let mut i = 0;
        while i <= Vx {
            self.memory[self.index as usize + i as usize] = self.registers[i as usize];
            i += 1;
        }
    }

    pub fn OP_Fx65(&mut self) {
        let Vx: u8 = ((self.opcode & 0x0F00) >> 8).try_into().unwrap();

        let mut i = 0;
        while i <= Vx {
            self.registers[i as usize] = self.memory[self.index as usize + i as usize];
            i += 1;
        }
    }

    pub fn cycle(&mut self) {
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[self.pc as usize + 1] as u16;

        self.pc += 2;

        self.decode_n_exe();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn decode_n_exe(&mut self) {
        match (self.opcode & 0xF000) >> 12 {
            0x8 => {
                match self.opcode & 0x000F {
                    0x0 => self.OP_8xy0(),
                    0x1 => self.OP_8xy1(),
                    0x2 => self.OP_8xy2(),
                    0x3 => self.OP_8xy3(),
                    0x4 => self.OP_8xy4(),
                    0x5 => self.OP_8xy5(),
                    0x6 => self.OP_8xy6(),
                    0x7 => self.OP_8xy7(),
                    0xE => self.OP_8xyE(),
                    _ => eprintln!("Erroneous opcode : {}", self.opcode),
                }
            },
            0x0 => {
                match self.opcode & 0x000F {
                    0x0 => self.OP_00E0(),
                    0xE => self.OP_00EE(),
                    _ => eprintln!("Erroneous opcode : {}", self.opcode),
                }
            },
            0xE => {
                match self.opcode & 0x000F {
                    0x1 => self.OP_ExA1(),
                    0xE => self.OP_Ex9E(),
                    _ => eprintln!("Erroneous opcode : {}", self.opcode),
                }
            },
            0xF => {
                match self.opcode & 0x00FF {
                    0x07 => self.OP_Fx07(),
                    0x0A => self.OP_Fx0A(),
                    0x15 => self.OP_Fx15(),
                    0x18 => self.OP_Fx18(),
                    0x1E => self.OP_Fx1E(),
                    0x29 => self.OP_Fx29(),
                    0x33 => self.OP_Fx33(),
                    0x55 => self.OP_Fx55(),
                    0x65 => self.OP_Fx65(),
                    _ => eprintln!("Erroneous opcode : {}", self.opcode),
                }
            },
            _ => eprintln!("Erroneous opcode : {}", self.opcode),

        }
    }
}

#[cfg(test)]
mod tests {

}
