use super::core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, EventPump, Sdl};

const SCALE_FACTOR: u32 = 20;

pub struct Displayer {
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl Displayer {
    pub fn new() -> Result<Displayer, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(
                "CHIP 8",
                SCREEN_WIDTH as u32 * SCALE_FACTOR,
                SCREEN_HEIGHT as u32 * SCALE_FACTOR,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Displayer {
            sdl_context,
            canvas,
        })
    }

    pub fn draw_background(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn draw(&mut self, pixels: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]) -> Result<(), String> {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(self.color(col));
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    fn color(&self, value: u8) -> Color {
        match value {
            0 => Color::BLACK,
            _ => Color::GREEN,
        }
    }

    pub fn events(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    pub fn keyset(&mut self) -> [bool; 16] {
        let keys: Vec<Keycode> = self
            .events()
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = true;
            }
        }

        chip8_keys
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.present();
    }
}
