const MEM_SIZE: usize = 0x1000;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Chip8 {
    mem: [u8; MEM_SIZE],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 { mem: [0; MEM_SIZE] }
    }
}
