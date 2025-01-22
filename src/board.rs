use crate::turn::PlayerTurn;

/// Slot availability
#[derive(Clone, Debug, PartialEq)]
pub enum Slot {
	Empty,
	PlayerOne,
	PlayerTwo,
}

/// Tic-tac-toe board
#[derive(Debug, Default)]
pub struct Board {
	/// Slots on board
	slots: Vec<Slot>,
}

impl Board {
	// TODO: Make click work...
	pub fn click(self, slot: usize, turn: PlayerTurn) -> Self {
		// Determine whose turn it is
		if self.slot_available(slot) {
			match turn {
				PlayerTurn::PlayerOne => {
					// Update slots
					let new_slots = &mut self.get_slots();
					new_slots[slot] = Slot::PlayerOne;

					println!("{:?}", new_slots);

					// Update display

					Self {
						slots: new_slots.to_vec(),
					 }
				},
				PlayerTurn::PlayerTwo => {
					let new_slots = &mut self.get_slots();
					new_slots[slot] = Slot::PlayerTwo;

					println!("{:?}", new_slots);

					// Update display

					Self {
						slots: new_slots.to_vec(),
					 }
				},
				PlayerTurn::GameOver => {
					// Do nothing (since game is over)
					Self {
						slots: self.slots,
					}
				},
			}
		} else {
			// Do nothing (since not a vaild slot)
			Self {
				slots: self.slots,
			}
		}
	}

	/// Get value of slots
	pub fn get_slots(&self) -> Vec<Slot> {
		self.slots.clone()
	}

	/// Create an empty board
	pub fn new() -> Board {
		let mut slots = Vec::new();

		for _ in 0..9 {
			slots.push(Slot::Empty);
		}

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
