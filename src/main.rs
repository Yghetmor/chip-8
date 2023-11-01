mod chip;
mod platform;

use crate::chip::Chip8;
use crate::platform::Platform;

fn main() {
    let chip = Chip8::new();
    Platform::new();
}
