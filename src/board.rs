use crate::turn::PlayerTurn;

/// Slot availability
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Slot {
	Empty,
	PlayerOne,
	PlayerTwo,
}

/// Tic-tac-toe board
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
	/// Slots on board
	slots: [Slot; 9],
}

impl Board {
	pub fn place(self, slot: usize, turn: &PlayerTurn) -> Self {
		// Confirm slot is available
		if self.slot_available(slot) {
			// Determine whose turn it is
			match turn {
				PlayerTurn::PlayerOne => {
					// Update slots
					let new_slots = &mut self.get_slots();
					new_slots[slot] = Slot::PlayerOne;

					// Update display
					Self { slots: *new_slots }
				}
				PlayerTurn::PlayerTwo => {
					// Update slots
					let new_slots = &mut self.get_slots();
					new_slots[slot] = Slot::PlayerTwo;

					// Update display
					Self { slots: *new_slots }
				}
				PlayerTurn::GameOver => {
					// Do nothing (since game is over)
					Self { slots: self.slots }
				}
			}
		} else {
			// Do nothing (since not a valid slot)
			Self { slots: self.slots }
		}
	}

	/// Get value of slots
	pub fn get_slots(&self) -> [Slot; 9] {
		self.slots
	}

	/// Create an empty board
	pub fn new() -> Self {
		let slots = [Slot::Empty; 9];

		Self { slots }
	}

	/// Check if a slot is available
	pub fn slot_available(&self, slot: usize) -> bool {
		let slots = &self.get_slots();
		let checked_slot = &slots[slot];

		if *checked_slot == Slot::Empty {
			return true;
		}

		false
	}
}

impl Default for Board {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use crate::board::{Board, Slot};
	use crate::turn::PlayerTurn;

	/// Click util function
	fn place(slot: usize, turn: PlayerTurn) -> Slot {
		let mut board = Board::new();

		board = board.place(slot, &turn);

		board.get_slots()[slot]
	}

	/// Test clicking slots
	#[test]
	fn place_test() {
		for i in 0..9 {
			assert_eq!(Slot::PlayerOne, place(i, PlayerTurn::PlayerOne));
			assert_eq!(Slot::PlayerTwo, place(i, PlayerTurn::PlayerTwo));
		}
	}
}
