use tic_tac_toe::board::Board;
use tic_tac_toe::click::click;
use tic_tac_toe::renderer::Renderer;
use tic_tac_toe::turn::PlayerTurn;
use tic_tac_toe::winner::{check_winner, Winner};

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
	let mut turn = PlayerTurn::PlayerOne;
	let mut board = Board::new();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::KeyDown {
					keycode: Some(keycode),
					..
				} => match keycode {
					Keycode::Escape => {
						break 'running;
					}
					Keycode::R => {
						board = Board::new();
						turn = PlayerTurn::PlayerOne;
						renderer.draw(&board)?;
						continue 'running;
					},
					_ => {},
				},
				Event::MouseButtonDown { x, y, .. } => {
					board = click(&board, &turn, x, y);

					let winner = check_winner(&board);

					if winner.winner != Winner::None {
						turn = PlayerTurn::GameOver;
					}

					match turn {
						PlayerTurn::PlayerOne => {
							turn = PlayerTurn::PlayerTwo;
						}
						PlayerTurn::PlayerTwo => {
							turn = PlayerTurn::PlayerOne;
						}
						PlayerTurn::GameOver => match winner.winner {
							Winner::PlayerOne => {
								println!("Player 1 wins!");
								break 'running;
							}
							Winner::PlayerTwo => {
								println!("Player 2 wins!");
								break 'running;
							}
							Winner::None => {},
						},
					}
				}
				_ => {}
			}

			renderer.draw(&board)?;
		}
	}

	Ok(())
}
