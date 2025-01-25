use crate::board::{Board, Slot};

#[derive(Debug, PartialEq)]
pub struct WinnerData {
	pub winner: Winner,
	pub line: Line,
}

#[derive(Debug, PartialEq)]
pub enum Line {
	ZeroHorizontal,
	OneHorizontal,
	TwoHorizontal,
	ZeroVertical,
	OneVertical,
	TwoVertical,
	ZeroDiagonal,
	OneDiagonal,
	None,
}

#[derive(Debug, PartialEq)]
pub enum Winner {
	PlayerOne,
	PlayerTwo,
	None,
}

/// Check **all postions** for a winner
pub fn check_winner(board: Board) -> WinnerData {
	let mut winner_data: WinnerData;

	winner_data = check_lines_horizontal(&board);
	if winner_data.winner != Winner::None {
		return winner_data;
	}

	winner_data = check_lines_vertical(&board);
	if winner_data.winner != Winner::None {
		return winner_data;
	}

	winner_data = check_lines_diagonal(&board);
	if winner_data.winner != Winner::None {
		return winner_data;
	}

	WinnerData {
		winner: Winner::None,
		line: Line::None,
	}
}

/// Check a line for a winner horizontally
fn check_line_horizontal(board: &Board, line: u8) -> WinnerData {
	match line {
		0 => {
			let slots = board.get_slots();

			if board.slot_available(0) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[0] == slots[1] && slots[0] == slots[2] {
				match slots[0] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::ZeroHorizontal,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::ZeroHorizontal,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		1 => {
			let slots = board.get_slots();

			if board.slot_available(4) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[3] == slots[4] && slots[3] == slots[5] {
				match slots[3] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::OneHorizontal,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::OneHorizontal,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		2 => {
			let slots = board.get_slots();

			if board.slot_available(6) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[6] == slots[7] && slots[6] == slots[8] {
				match slots[6] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::TwoHorizontal,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::TwoHorizontal,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		_ => WinnerData {
			winner: Winner::None,
			line: Line::None,
		},
	}
}

/// Check **all** lines for a winner horizontally
fn check_lines_horizontal(board: &Board) -> WinnerData {
	// Loop through all lines to check
	for i in 0..3 {
		let winner_data = check_line_horizontal(board, i);
		if winner_data.winner != Winner::None {
			return winner_data;
		}
	}

	WinnerData {
		winner: Winner::None,
		line: Line::None,
	}
}

/// Check a line for a winner vertically
fn check_line_vertical(board: &Board, line: u8) -> WinnerData {
	match line {
		0 => {
			let slots = board.get_slots();

			if board.slot_available(0) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[0] == slots[3] && slots[0] == slots[6] {
				match slots[0] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::ZeroVertical,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::ZeroVertical,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		1 => {
			let slots = board.get_slots();

			if board.slot_available(1) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[1] == slots[4] && slots[1] == slots[7] {
				match slots[1] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::OneVertical,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::OneVertical,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		2 => {
			let slots = board.get_slots();

			if board.slot_available(2) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[2] == slots[5] && slots[2] == slots[8] {
				match slots[2] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::TwoVertical,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::TwoVertical,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		_ => WinnerData {
			winner: Winner::None,
			line: Line::None,
		},
	}
}

/// Check **all** lines for a winner vertically
fn check_lines_vertical(board: &Board) -> WinnerData {
	// Loop through all lines to check
	for i in 0..3 {
		let winner_data = check_line_vertical(board, i);
		if winner_data.winner != Winner::None {
			return winner_data;
		}
	}

	WinnerData {
		winner: Winner::None,
		line: Line::None,
	}
}

/// Check a line diagonally
fn check_line_diagonal(board: &Board, line: u8) -> WinnerData {
	match line {
		0 => {
			let slots = board.get_slots();

			if board.slot_available(0) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[0] == slots[4] && slots[0] == slots[8] {
				match slots[0] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::ZeroDiagonal,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::ZeroDiagonal,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		1 => {
			let slots = board.get_slots();

			if board.slot_available(2) {
				return WinnerData {
					winner: Winner::None,
					line: Line::None,
				};
			}

			if slots[2] == slots[4] && slots[2] == slots[6] {
				match slots[2] {
					Slot::PlayerOne => {
						return WinnerData {
							winner: Winner::PlayerOne,
							line: Line::OneDiagonal,
						};
					}
					Slot::PlayerTwo => {
						return WinnerData {
							winner: Winner::PlayerTwo,
							line: Line::OneDiagonal,
						};
					}
					_ => {}
				}
			}

			WinnerData {
				winner: Winner::None,
				line: Line::None,
			}
		}
		_ => WinnerData {
			winner: Winner::None,
			line: Line::None,
		},
	}
}

/// Check **all** lines diagonally
fn check_lines_diagonal(board: &Board) -> WinnerData {
	// Loop through all lines to check
	for i in 0..2 {
		let winner_data = check_line_diagonal(board, i);
		if winner_data.winner != Winner::None {
			return winner_data;
		}
	}

	WinnerData {
		winner: Winner::None,
		line: Line::None,
	}
}

#[cfg(test)]
mod tests {
	use crate::board::Board;
	use crate::turn::PlayerTurn;
	use crate::winner::{check_winner, Line, Winner, WinnerData};

	#[test]
	fn zero_horizonal() {
		let mut board = Board::new();

		// Setup win with ZeroHorizonal
		board = board.click(0, PlayerTurn::PlayerOne);
		board = board.click(1, PlayerTurn::PlayerOne);
		board = board.click(2, PlayerTurn::PlayerOne);

		// Get winner data
		let winner_data = check_winner(board);

		assert_eq!(
			WinnerData {
				winner: Winner::PlayerOne,
				line: Line::ZeroHorizontal,
			},
			winner_data
		)
	}

	#[test]
	fn one_horizonal() {
		let mut board = Board::new();

		// Setup win with OneHorizonal
		board = board.click(3, PlayerTurn::PlayerOne);
		board = board.click(4, PlayerTurn::PlayerOne);
		board = board.click(5, PlayerTurn::PlayerOne);

		// Get winner data
		let winner_data = check_winner(board);

		assert_eq!(
			WinnerData {
				winner: Winner::PlayerOne,
				line: Line::OneHorizontal,
			},
			winner_data
		)
	}

	#[test]
	fn two_horizonal() {
		let mut board = Board::new();

		// Setup win with TwoHorizonal
		board = board.click(6, PlayerTurn::PlayerOne);
		board = board.click(7, PlayerTurn::PlayerOne);
		board = board.click(8, PlayerTurn::PlayerOne);

		// Get winner data
		let winner_data = check_winner(board);

		assert_eq!(
			WinnerData {
				winner: Winner::PlayerOne,
				line: Line::TwoHorizontal,
			},
			winner_data
		)
	}

	#[test]
	fn zero_vertical() {
		let mut board = Board::new();

		// Setup win with ZeroVertical
		board = board.click(0, PlayerTurn::PlayerOne);
		board = board.click(3, PlayerTurn::PlayerOne);
		board = board.click(6, PlayerTurn::PlayerOne);

		// Get winner data
		let winner_data = check_winner(board);

		assert_eq!(
			WinnerData {
				winner: Winner::PlayerOne,
				line: Line::ZeroVertical,
			},
			winner_data
		)
	}

	#[test]
	fn one_vertical() {
		let mut board = Board::new();

		// Setup win with OneVertical
		board = board.click(1, PlayerTurn::PlayerOne);
		board = board.click(4, PlayerTurn::PlayerOne);
		board = board.click(7, PlayerTurn::PlayerOne);

		// Get winner data
		let winner_data = check_winner(board);

		assert_eq!(
			WinnerData {
				winner: Winner::PlayerOne,
				line: Line::OneVertical,
			},
			winner_data
		)
	}
}
