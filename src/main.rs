use sdl2::event::Event;
use std::env;
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

    chip.load_rom(&rom);
    displayer.draw();

    'running: loop {
        chip.emulate_cycle();

        for event in displayer.events().poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { .. } => break 'running,
                _ => {}
            }
        }

        displayer.keyset();
    }

    Ok(())
}
