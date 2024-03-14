use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::thread;
use std::time::Duration;
use yac8::chip::core::Chip8;
use yac8::chip::display::Displayer;

fn main() -> Result<(), String> {
    let args = env::args();
    if args.len() < 2 {
        return Err("Incorrect amount of arguments provided".to_string());
    }

    let rom = args.last().unwrap();
    let mut chip = Chip8::new();
    let mut displayer = Displayer::new()?;
    let mut cpu_inst_exec = 0;
    let sleep_duration = 1;

    chip.load_rom(&rom);

    'running: loop {
        chip.emulate_cycle();

        for event in displayer.events().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if chip.screen_drawed {
            displayer.draw(&chip.screen)?;
        }

        chip.keypad = displayer.keyset();
        cpu_inst_exec += 1;
        if cpu_inst_exec == 60 {
            thread::sleep(Duration::from_millis(sleep_duration));
        }
    }

    Ok(())
}
