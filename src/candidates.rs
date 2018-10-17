const ALL_CANDIDATES: u16 = 0b111111111;
const NINE: 	u16 = 0b100000000;
const EIGHT: 	u16 = 0b010000000;
const SEVEN: 	u16 = 0b001000000;
const SIX: 		u16 = 0b000100000;
const FIVE: 	u16 = 0b000010000;
const FOUR: 	u16 = 0b000001000;
const THREE: 	u16 = 0b000000100;
const TWO: 		u16 = 0b000000010;
const ONE: 		u16 = 0b000000001;

#[derive(Clone, Copy)]
pub struct Candidates {
	bitmask: u16,
}

impl Candidates {
	pub fn new_default() -> Candidates {
		Candidates {
			bitmask: ALL_CANDIDATES,
		}
	}

	pub fn new(value: u8) -> Candidates {
		Candidates {
			bitmask: match value {
				9 => NINE,
				8 => EIGHT,
				7 => SEVEN,
				6 => SIX,
				5 => FIVE,
				4 => FOUR,
				3 => THREE,
				2 => TWO,
				1 => ONE,
				_ => ALL_CANDIDATES,
			}
		}
	}

	pub fn value(&self) -> Option<u8> {
		match self.bitmask {
			NINE 	=> Some(9),
			EIGHT 	=> Some(8),
			SEVEN 	=> Some(7),
			SIX 	=> Some(6),
			FIVE 	=> Some(5),
			FOUR 	=> Some(4),
			THREE 	=> Some(3),
			TWO 	=> Some(2),
			ONE 	=> Some(1),
			_ => None,
		}
	}
}
