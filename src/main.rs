use tic_tac_toe::board::Board;
use tic_tac_toe::renderer::Renderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("Tic Tac Toe", 1200, 1200)
		.position_centered()
		.build()
		.map_err(|error| error.to_string())?;

	let mut renderer = Renderer::new(window)?;
	let mut event_pump = sdl_context.event_pump().unwrap();

	let board = Board::new();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::KeyDown {
					keycode: Some(keycode),
					..
				} => match keycode {
					Keycode::Escape => break 'running,
					_ => {}
				},
				_ => {}
			}

			renderer.draw(&board)?;
		}
	}

	Ok(())
}
