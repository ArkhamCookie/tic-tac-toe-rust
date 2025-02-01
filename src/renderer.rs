use sdl2::pixels::Color;
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
	fn draw_horizontal_line(&mut self, point: &Point) -> Result<(), String> {
		let Point(x, y) = point;

		self.canvas.fill_rect(Rect::new(
			*x,
			*y,
			1200,
			10,
		))?;

		Ok(())
	}

	/// Draw a vertical line
	fn draw_vertical_line(&mut self, point: &Point) -> Result<(), String> {
		let Point(x, y) = point;

		self.canvas.set_draw_color(Color::WHITE);
		self.canvas.fill_rect(Rect::new(
			*x,
			*y,
			10,
			1200,
		))?;

		Ok(())
	}

	/// Draw the board
	fn draw_board(&mut self) -> Result<(), String> {
		self.draw_vertical_line(&Point(400, 0))?;
		self.draw_vertical_line(&Point(800, 0))?;

		self.draw_horizontal_line(&Point(0, 400))?;
		self.draw_horizontal_line(&Point(0, 800))?;

		Ok(())
	}

	/// Draw everything to screen
	pub fn draw(&mut self) -> Result<(), String> {
		self.draw_board()?;

		self.canvas.present();

		Ok(())
	}
}

