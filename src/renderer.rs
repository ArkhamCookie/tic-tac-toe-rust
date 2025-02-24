use crate::board::{Board, Slot};
use crate::turn::PlayerTurn;

use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

/// Point on screen
pub struct Point(pub i32, pub i32);

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
	fn draw_horizontal_line(&mut self, length: u32, point: &Point) -> Result<(), String> {
		let Point(x, y) = point;

		self.canvas.fill_rect(Rect::new(*x, *y, length, 10))?;

		Ok(())
	}

	/// Draw a vertical line
	fn draw_vertical_line(&mut self, length: u32, point: &Point) -> Result<(), String> {
		let Point(x, y) = point;

		self.canvas.set_draw_color(Color::WHITE);
		self.canvas.fill_rect(Rect::new(*x, *y, 10, length))?;

		Ok(())
	}

	/// Draw the board
	fn draw_board(&mut self) -> Result<(), String> {
		self.draw_vertical_line(1000, &Point(400, 100))?;
		self.draw_vertical_line(1000, &Point(800, 100))?;

		self.draw_horizontal_line(1000, &Point(100, 400))?;
		self.draw_horizontal_line(1000, &Point(100, 800))?;

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
			0 => {
				self.canvas.fill_rect(Rect::new(150, 150, 100, 100))?;
			}
			1 => {
				self.canvas.fill_rect(Rect::new(550, 150, 100, 100))?;
			}
			2 => {
				self.canvas.fill_rect(Rect::new(950, 150, 100, 100))?;
			}
			3 => {
				self.canvas.fill_rect(Rect::new(150, 550, 100, 100))?;
			}
			4 => {
				self.canvas.fill_rect(Rect::new(550, 550, 100, 100))?;
			}
			5 => {
				self.canvas.fill_rect(Rect::new(950, 550, 100, 100))?;
			}
			6 => {
				self.canvas.fill_rect(Rect::new(150, 950, 100, 100))?;
			}
			7 => {
				self.canvas.fill_rect(Rect::new(550, 950, 100, 100))?;
			}
			8 => {
				self.canvas.fill_rect(Rect::new(950, 950, 100, 100))?;
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
