use crate::chip::font::FONT_SET;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

const MEM_SIZE: usize = 0x1000;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
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
    pub keypad: [bool; 0x10],
    pub screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    // auxiliar variables
    await_key: bool,
    keypad_reg: usize,
    pub screen_drawed: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0; MEM_SIZE];

        ram[..FONT_SET.len()].copy_from_slice(&FONT_SET);

        Chip8 {
            i: 0,
            pc: 0x200,
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
            await_key: false,
            keypad_reg: 0,
            screen_drawed: false,
        }
    }

    pub fn load_rom(&mut self, rom: &str) {
        let mut f = File::open(rom).unwrap();
        let mut buffer = [0u8; 3588];

        let _size = match f.read(&mut buffer) {
            Ok(bytes_size) => bytes_size,
            _ => 0,
        };

        // load program to memory
        let mut addr = self.pc;
        for byte in buffer {
            if addr > 4095 {
                break;
            }
            self.ram[addr] = byte;
            addr += 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch(self.pc);
        self.execute(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            // if self.sound_timer == 1 {

            // }
            self.sound_timer -= 1;
        }
    }

    fn fetch(&self, addr: usize) -> usize {
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

    pub fn execute(&mut self, opcode: usize) {
        let nibbles = self.to_nibbles(opcode);

        // function parameters
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5ky0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0E) => self.op_8xye(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0A, _, _, _) => self.op_annn(nnn),
            (0x0B, _, _, _) => self.op_bnnn(nnn),
            (0x0C, _, _, _) => self.op_cxkk(x, kk),
            (0x0D, _, _, _) => self.op_dxyn(x, y, n),
            (0x0E, _, 0x09, 0x0E) => self.op_ex9e(x),
            (0x0E, _, 0x0A, 0x01) => self.op_exa1(x),
            (0x0F, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0F, _, 0x00, 0x0A) => self.op_fx0a(x),
            (0x0F, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0F, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0F, _, 0x01, 0x0E) => self.op_fx1e(x),
            (0x0F, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0F, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0F, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0F, _, 0x06, 0x05) => self.op_fx65(x),
            _ => self.pc += OPCODE_SIZE,
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
        self.sp += 1;
        self.pc = addr;
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
    fn op_6xkk(&mut self, x: usize, kk: u8) {
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

    // SUBN - if Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        self.regs[0x0F] = if vy > vx { 1 } else { 0 };
        self.regs[x] = vy - vx;
        self.pc += OPCODE_SIZE;
    }

    // SHL - If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn op_8xye(&mut self, x: usize) {
        let vx = self.regs[x];
        self.regs[0x0F] = (vx & 0x80) >> 7;
        self.regs[x] = vx << 1;
        self.pc += OPCODE_SIZE;
    }

    // SNE - Skip next instruction if Vx != Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        let vx = self.regs[x];
        let vy = self.regs[y];
        if vx != vy {
            self.pc += 2 * OPCODE_SIZE;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }

    // LD - Set I register with the value of nnn
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
        self.pc += OPCODE_SIZE;
    }

    // JP - Jump to location nnn + V0
    fn op_bnnn(&mut self, nnn: usize) {
        let addr = self.regs[0x00] as usize + nnn;
        self.pc = addr;
    }

    // RND - Set Vx = random Byte AND kk
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        let rand_num: u8 = rng.gen_range(0..255);
        self.regs[x] = rand_num & kk;
        self.pc += OPCODE_SIZE;
    }

    // DRW - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    // The interpreter reads n bytes from memory, starting at the address
    // stored in I. These bytes are then displayed as sprites on screen at
    // coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise
    // it is set to 0. If the sprite is positioned so part of it is outside
    // the coordinates of the display, it wraps around to the opposite side
    // of the screen.
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.regs[0x0F] = 0;
        for byte in 0..n {
            let sprite = self.ram[self.i + byte];
            let cy = (self.regs[y] as usize + byte) % SCREEN_HEIGHT;
            for bit in 0..8 {
                // for each bit from the most significant to the lest
                let cx = (self.regs[x] as usize + bit) % SCREEN_WIDTH;
                let pixel = sprite >> (7 - bit) & 1;
                self.regs[0x0F] |= pixel & self.screen[cy][cx];
                self.screen[cy][cx] = pixel ^ self.screen[cy][cx];
            }
        }
        self.screen_drawed = true;
        self.pc += OPCODE_SIZE;
    }

    // SKP - Skip next instruction if key with the value of Vx is pressed
    fn op_ex9e(&mut self, x: usize) {
        let vx = self.regs[x] as usize;
        if self.keypad[vx] {
            self.pc += 2 * OPCODE_SIZE;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }

    // SKNP - Skip next instruction if key with the value of Vx is not pressed
    fn op_exa1(&mut self, x: usize) {
        let vx = self.regs[x] as usize;
        if !self.keypad[vx] {
            self.pc += 2 * OPCODE_SIZE;
        } else {
            self.pc += OPCODE_SIZE;
        }
    }

    // LD - Load the value dt of into Vx
    fn op_fx07(&mut self, x: usize) {
        self.regs[x] = self.dt as u8;
        self.pc += OPCODE_SIZE;
    }

    // LD - Execution stops until a key is pressed
    fn op_fx0a(&mut self, x: usize) {
        self.await_key = true;
        self.keypad_reg = x;
        self.pc += OPCODE_SIZE;
    }

    // LD - Load the value Vx into Dt
    fn op_fx15(&mut self, x: usize) {
        self.dt = self.regs[x] as usize;
        self.pc += OPCODE_SIZE;
    }

    // LD - St is equal to the value of Vx
    fn op_fx18(&mut self, x: usize) {
        self.st = self.regs[x] as usize;
        self.pc += OPCODE_SIZE;
    }

    // ADD - Add the vaues of I and Vx and stores it in I
    fn op_fx1e(&mut self, x: usize) {
        self.i = self.i + self.regs[x] as usize;
        self.pc += OPCODE_SIZE;
    }

    // LD - Stores the value of Vx that indicates the position of hex sprite in I
    fn op_fx29(&mut self, x: usize) {
        self.i = (self.regs[x] as usize) * 5;
        self.pc += OPCODE_SIZE;
    }

    // LD - Store BCD representation of Vx in memory locations I, I+1, I+2
    fn op_fx33(&mut self, x: usize) {
        let vx = self.regs[x];
        self.ram[self.i] = vx / 100;
        self.ram[self.i + 1] = (vx % 100) / 10;
        self.ram[self.i + 2] = vx % 10;
        self.pc += OPCODE_SIZE;
    }

    // LD - Store registers V0 through Vx in memory starting at location I
    fn op_fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.ram[self.i + i] = self.regs[i];
        }
        self.pc += OPCODE_SIZE;
    }

    // LD - Store registers V0 through Vx in memory starting at location I
    fn op_fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.regs[i] = self.ram[self.i + i];
        }
        self.pc += OPCODE_SIZE;
    }
}
