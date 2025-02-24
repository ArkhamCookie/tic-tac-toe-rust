use crate::board::Board;
use crate::turn::PlayerTurn;

/// Run when a player clicks the board
pub fn click(
	board: &Board,
	turn: &PlayerTurn,
	x_coord: i32,
	y_coord: i32,
) -> Result<Board, String> {
	let slot = get_slot(x_coord, y_coord).unwrap();

	if !board.slot_available(slot) {
		return Err(String::from("slot not available"));
	}

	Ok(board.clone().place(slot, turn))
}

/// Get slot based on coords
fn get_slot(x_coord: i32, y_coord: i32) -> Option<usize> {
	if x_coord > 0 && x_coord < 400 {
		if y_coord > 0 && y_coord < 395 {
			Some(0)
		} else if y_coord > 410 && y_coord < 800 {
			Some(3)
		} else if y_coord > 810 && y_coord < 1100 {
			Some(6)
		} else {
			None
		}
	} else if x_coord > 410 && x_coord < 800 {
		if y_coord > 0 && y_coord < 395 {
			Some(1)
		} else if y_coord > 410 && y_coord < 800 {
			Some(4)
		} else if y_coord > 810 && y_coord < 1100 {
			Some(7)
		} else {
			None
		}
	} else if x_coord > 810 && x_coord < 1100 {
		if y_coord > 0 && y_coord < 395 {
			Some(2)
		} else if y_coord > 410 && y_coord < 800 {
			Some(5)
		} else if y_coord > 810 && y_coord < 1100 {
			Some(8)
		} else {
			None
		}
	} else {
		None
	}
}
