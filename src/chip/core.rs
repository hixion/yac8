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

    pub fn fetch(&self, addr: usize) -> usize {
        // most significant byte
        let mbyte = self.ram[addr] as usize;
        // less significant byte
        let lbyte = self.ram[addr + 1] as usize;

        ((mbyte << 8) | lbyte) as usize
    }

    fn to_nibbles(&self, opcode: usize) -> (u8, u8, u8, u8) {
        (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        )
    }

    pub fn execute(&mut self) {
        let opcode: usize = self.fetch(self.pc);
        let nibbles = self.to_nibbles(opcode);

        // function parameters
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;

        match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5ky0(x, y),
            (0x06, _, _, _) => self.op_6kk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            _ => unreachable!(),
        }
    }

    // -------------------------------
    //           INSTRUCTIONS
    // -------------------------------

    // CLS - clear the display
    fn op_00e0(&mut self) {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.screen[i][j] = 0;
            }
        }
        self.pc += OPCODE_SIZE;
    }

    // RET - return from a subrutine
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    // JP - Jump to location nnn
    fn op_1nnn(&mut self, addr: usize) {
        self.pc = addr;
    }

    // CALL - call subrutine at nnn
    fn op_2nnn(&mut self, addr: usize) {
        self.stack[self.sp as usize] = self.pc + OPCODE_SIZE;
        self.pc = addr;
        self.sp += 1;
    }

    // SE - skip next instruction if Vx == kk
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        let vx = self.regs[x];
        if vx == kk {
            self.pc += 2 * OPCODE_SIZE;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }

    // SNE - skip next instruction if Vx != kk.
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        let vx = self.regs[x];
        if vx != kk {
            self.pc += 2 * OPCODE_SIZE;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }

    // SE - skip next instruction if Vx == Vy
    fn op_5ky0(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        if vx == vy {
            self.pc = 2 * OPCODE_SIZE;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }

    // LD - sets the value kk into Vx register
    fn op_6kk(&mut self, x: usize, kk: u8) {
        self.regs[x] = kk as u8;
        self.pc += OPCODE_SIZE;
    }

    // ADD - adds the value kk to Vx register
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        let vx = self.regs[x];
        self.regs[x] = vx + kk;
        self.pc += OPCODE_SIZE;
    }

    // LD - sets the value Vy register into Vx register
    fn op_8xy0(&mut self, x: usize, y: usize) {
        let vy = self.regs[y];
        self.regs[x] = vy;
        self.pc += OPCODE_SIZE;
    }

    // OR - Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx
    fn op_8xy1(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        self.regs[x] = vx | vy;
        self.pc += OPCODE_SIZE;
    }

    // AND - Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx
    fn op_8xy2(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        self.regs[x] = vx & vy;
        self.pc += OPCODE_SIZE;
    }

    // XOR - Performs a bitwise XOR on the values of Vx and Vy, then stores the result in Vx
    fn op_8xy3(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        self.regs[x] = vx ^ vy;
        self.pc += OPCODE_SIZE;
    }

    // ADD - Adds the values of Vx and Vy, checks overflow and then stores the result in Vx
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.regs[x] as u16;
        let vy = self.regs[y] as u16;
        let result = vx + vy;
        self.regs[0x0F] = if result > 0xFF { 1 } else { 0 };
        self.regs[x] = result as u8;
        self.pc += OPCODE_SIZE;
    }

    // SUB - if Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        self.regs[0x0F] = if vx > vy { 1 } else { 0 };
        self.regs[x] = vx - vy;
        self.pc += OPCODE_SIZE;
    }

    // SHR -  If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_8xy6(&mut self, x: usize) {
        let vx = self.regs[x];
        let bit = vx & 0x01;
        self.regs[0x0F] = if bit == 1 { 1 } else { 0 };
        self.regs[x] = vx >> 1;
        self.pc += OPCODE_SIZE;
    }

    // SUB - if Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        self.regs[0x0F] = if vy > vx { 1 } else { 0 };
        self.regs[x] = vy - vx;
        self.pc += OPCODE_SIZE;
    }
}
