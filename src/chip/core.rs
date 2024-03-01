use super::errors::ChipError;
use crate::chip::font::FONT_SET;

const MEM_SIZE: usize = 0x1000;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const OPCODE_SIZE: usize = 2;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Chip8 {
    i: usize,  // index register
    pc: usize, // program counter
    sp: u8,    // stack pointer
    st: usize, // sound timer register
    dt: usize, // delay timer register
    ram: [u8; MEM_SIZE],
    regs: [u8; 0x10],
    delay_timer: u8,
    sound_timer: u8,
    stack: [usize; 0x10],
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

    fn fetch(&self, addr: usize) -> Result<usize, ChipError> {
        if addr > self.ram.len() {
            return Err(ChipError::AddressBoundaryError);
        }

        // most significant byte
        let mbyte = self.ram[addr as usize];
        // less significant byte
        let lbyte = self.ram[(addr + 1) as usize];

        Ok(((mbyte << 8) | lbyte) as usize)
    }

    fn to_nibbles(&self, opcode: usize) -> (u8, u8, u8, u8) {
        (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        )
    }

    pub fn execute(&mut self) -> Result<(), ChipError> {
        let opcode: usize = self.fetch(self.pc)?;
        let nibbles = self.to_nibbles(opcode);

        // function parameters
        let nnn = (opcode & 0x0FFF) as usize;
        let x = (opcode & 0x0F00) as usize;
        let kk = (opcode & 0x00FF) as usize;

        match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            _ => Err(ChipError::InvalidOpcode),
        }
    }

    // -------------------------------
    //           INSTRUCTIONS
    // -------------------------------

    // CLS - clear the display
    fn op_00e0(&mut self) -> Result<(), ChipError> {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.screen[i][j] = 0;
            }
        }

        self.pc += OPCODE_SIZE;
        Ok(())
    }

    // RET - return from a subrutine
    fn op_00ee(&mut self) -> Result<(), ChipError> {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        Ok(())
    }

    // JP - Jump to location nnn
    fn op_1nnn(&mut self, addr: usize) -> Result<(), ChipError> {
        self.pc = addr;
        Ok(())
    }

    // CALL - call subrutine at nnn
    fn op_2nnn(&mut self, addr: usize) -> Result<(), ChipError> {
        self.stack[self.sp as usize] = self.pc + OPCODE_SIZE;
        self.pc = addr;
        self.sp += 1;
        Ok(())
    }

    // SE - skip next instruction if Vx == kk
    fn op_3xkk(&mut self, x: usize, kk: usize) -> Result<(), ChipError> {
        let vx = self.regs[x];
        if vx as usize == kk {
            self.pc += 2 * OPCODE_SIZE;
        }
        Ok(())
    }
}
