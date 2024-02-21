use super::errors::ChipError;
use crate::chip::font::FONT_SET;

const MEM_SIZE: usize = 0x1000;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Chip8 {
    i: u16,    // index register
    pc: u16,   // program counter
    sp: u8,    // stack pointer
    st: usize, // sound timer register
    dt: usize, // delay timer register
    ram: [u8; MEM_SIZE],
    regs: [u8; 0x10],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 0x10],
    keypad: [bool; 0x10],
    screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0; MEM_SIZE];

        for i in 0..FONT_SET.len() {
            ram[i] = FONT_SET[i];
        }

        Chip8 {
            i: 0,
            pc: 0,
            sp: 0,
            st: 0,
            dt: 0,
            ram,
            regs: [0; 0x10],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 0x10],
            keypad: [false; 0x10],
            screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn fetch(&self, addr: u16) -> Result<u16, ChipError> {
        if addr > self.ram.len() as u16 {
            return Err(ChipError::AddressBoundaryError);
        }

        // most significant byte
        let mbyte: u8 = self.ram[addr as usize];
        // less significant byte
        let lbyte: u8 = self.ram[(addr + 1) as usize];

        Ok(((mbyte << 8) | lbyte) as u16)
    }

    pub fn execute(&mut self) -> Result<(), ChipError> {
        let instr: u16 = self.fetch(self.pc)?;

        match instr {
            0x00E0 => self.op_00e0(),
            _ => Err(ChipError::InvalidOpcode),
        }
    }

    fn op_00e0(&mut self) -> Result<(), ChipError> {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.screen[i][j] = 0;
            }
        }

        self.pc += 1;
        Ok(())
    }
}
