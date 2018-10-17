use candidates::Candidates;

pub enum Contents {
	Value(u8),
	Empty,
}

pub struct Cell<'a> {
	candidates: &'a Candidates,
	pub contents: Contents,
}

impl<'a> Cell<'a> {
	pub fn new(candidates: &'a Candidates) -> Cell {
		Cell {
			candidates: candidates,
			contents: match candidates.value {
				1...9 => Contents::Value(candidates.value),
				_ => Contents::Empty,
			}
		}
	}
}
