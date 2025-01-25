use crate::board::{Board, Slot};

pub struct WinnerData {
	pub winner: Winner,
	pub line: Line,
}

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

#[derive(PartialEq)]
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
				match slots[4] {
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
							line: Line::OneHorizontal,
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
	for i in 0..2 {
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
				match slots[0] {
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
				match slots[0] {
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
	for i in 0..2 {
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
	for i in 0..1 {
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
