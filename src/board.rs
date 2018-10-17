use coordinate::Coordinate;
use candidates::Candidates;
use cell::*;
use std::fmt::Display;

pub struct Board<'a> {
	answered_count: u8,
	cells: [[Cell<'a>; 9]; 9],
	fresh_coordinates: Vec<Coordinate>,
}

impl<'a> Board<'a> {
	pub fn new(raw_values: Vec<Vec<i32>>) -> Board<'a> {
		let board = Board {
			answered_count: 0,
			cells: [[Cell::new(Candidates::new_default()); 9]; 9],
			fresh_coordinates: vec![],
		};

		for (i, row) in raw_values.into_iter().enumerate() {
			for (j, value) in row.into_iter().enumerate() {
				let candidates = Candidates::new(value);
				let cell = Cell::new(candidates);

				board.update_cell(cell, Coordinate{ x: j, y: i });
			}
		}

		board
	}

	pub fn is_solved(&self) -> bool {
		self.answered_count >= 81
	}

	pub fn update_cell(&self, coordinate: Coordinate, cell: Cell) {
		if self.cells[coordinate.y][coordinate.x].contents == Contents::Value { return }

		self.cells[coordinate.y][coordinate.x] = cell;

		if cell.contents == Contents::Value {
			self.answered_count += 1;
			self.fresh_coordinates.push(coordinate);
		}
	}
}

impl<'a> Display for Board<'a> {
	fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
	    for row in &self.values {
	    	for value in row {
	    		write!(f, "{} ", value);
	    	}
	    	writeln!(f);
	    	writeln!(f);
	    }

	    Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn board_initializer_test() {
		let initial_values = vec![
			vec![1, 2, 3, 4, 5, 6, 7, 8, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
		];
		let mut board = Board::new(initial_values);

		assert_eq!(8, board.answered_count);
		assert_eq!(format!("{:x}", 0x01FF), format!("{:x}", board.candidates[3][6]));
		assert_eq!(format!("{:x}", 0x0004), format!("{:x}", board.candidates[0][2]));
	}
}
