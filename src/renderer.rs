use crate::board::{Board, Slot};
use crate::turn::PlayerTurn;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

/// Renderer for screen
pub struct Renderer {
	canvas: WindowCanvas,
}

impl Renderer {
	/// Create a new screen
	pub fn new(window: Window) -> Result<Renderer, String> {
		let canvas = window
			.into_canvas()
			.build()
			.map_err(|error| error.to_string())?;

		Ok(Renderer { canvas })
	}

	/// Draw a horizontal line
	fn draw_horizontal_line(&mut self, length: u32, point: &SDLPoint) -> Result<(), String> {
		let x = point.x;
		let y = point.y;

		self.canvas.fill_rect(Rect::new(x, y, length, 10))?;

		Ok(())
	}

	/// Draw a vertical line
	fn draw_vertical_line(&mut self, length: u32, point: &SDLPoint) -> Result<(), String> {
		let x = point.x;
		let y = point.y;

		self.canvas.set_draw_color(Color::WHITE);
		self.canvas.fill_rect(Rect::new(x, y, 10, length))?;

		Ok(())
	}

	/// Draw the board
	fn draw_board(&mut self) -> Result<(), String> {
		self.draw_vertical_line(1000, &SDLPoint::new(400, 100))?;
		self.draw_vertical_line(1000, &SDLPoint::new(800, 100))?;

		self.draw_horizontal_line(1000, &SDLPoint::new(100, 400))?;
		self.draw_horizontal_line(1000, &SDLPoint::new(100, 800))?;

		Ok(())
	}

	/// Draw a circle piece
	fn draw_cirle(&mut self, center: SDLPoint) -> Result<(), String> {
		let radius = 100;

		let mut x = radius;
		let mut y = 0;

		let mut re = x * x + y * y - radius * radius;

		while x >= y {
			self.canvas
				.draw_point(SDLPoint::new(center.x + x, center.y + y))?;
			self.canvas
				.draw_point(SDLPoint::new(center.x + y, center.y + x))?;

			self.canvas
				.draw_point(SDLPoint::new(center.x + x, center.y - y))?;
			self.canvas
				.draw_point(SDLPoint::new(center.x + y, center.y - x))?;

			self.canvas
				.draw_point(SDLPoint::new(center.x - x, center.y + y))?;
			self.canvas
				.draw_point(SDLPoint::new(center.x - y, center.y + x))?;

			self.canvas
				.draw_point(SDLPoint::new(center.x - x, center.y - y))?;
			self.canvas
				.draw_point(SDLPoint::new(center.x - y, center.y - x))?;

			if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
				re += 1 - 2 * x;
				x -= 1;
			}

			re += 2 * y + 1;
			y += 1;
		}

		Ok(())
	}

	/// Draw cross piece
	fn draw_cross(&mut self, center: SDLPoint) -> Result<(), String> {
		self.canvas.draw_line(SDLPoint::new(center.x - 100, center.y - 125), SDLPoint::new(center.x + 100, center.y + 125))?;
		self.canvas.draw_line(SDLPoint::new(center.x + 100, center.y - 125), SDLPoint::new(center.x - 100, center.y + 125))?;

		Ok(())
	}

	/// Draw player's piece
	fn draw_player(&mut self, slot: usize, player: PlayerTurn) -> Result<(), String> {
		match player {
			PlayerTurn::PlayerOne => {
				self.canvas.set_draw_color(Color::BLUE);
			}
			PlayerTurn::PlayerTwo => {
				self.canvas.set_draw_color(Color::RED);
			}
			_ => {}
		}

		match slot {
			0 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(250, 250))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(250, 250))?;
				}
				_ => {}
			},
			1 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(600, 250))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(600, 250))?;
				}
				_ => {}
			}
			2 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(950, 250))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(950, 250))?;
				}
				_ => {}
			}
			3 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(250, 600))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(250, 600))?;
				}
				_ => {}
			}
			4 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(600, 600))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(600, 600))?;
				}
				_ => {}
			}
			5 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(950, 600))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(950, 600))?;
				}
				_ => {}
			}
			6 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(250, 950))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(250, 950))?;
				}
				_ => {}
			}
			7 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(600, 950))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(600, 950))?;
				}
				_ => {}
			}
			8 => match player {
				PlayerTurn::PlayerOne => {
					self.draw_cirle(SDLPoint::new(950, 950))?;
				}
				PlayerTurn::PlayerTwo => {
					self.draw_cross(SDLPoint::new(950, 950))?;
				}
				_ => {}
			}
			_ => {}
		}

		Ok(())
	}

	/// Draw the state of the game
	fn draw_state(&mut self, board: &Board) -> Result<(), String> {
		let slots = board.get_slots();

		for i in 0..9 {
			match slots[i] {
				Slot::PlayerOne => {
					self.draw_player(i, PlayerTurn::PlayerOne)?;
				}
				Slot::PlayerTwo => {
					self.draw_player(i, PlayerTurn::PlayerTwo)?;
				}
				Slot::Empty => {}
			}
		}

		Ok(())
	}

	/// Draw everything to screen
	pub fn draw(&mut self, board: &Board) -> Result<(), String> {
		self.canvas.set_draw_color(Color::BLACK);
		self.canvas.clear();

		self.draw_board()?;
		self.draw_state(board)?;

		self.canvas.present();

		Ok(())
	}
}
