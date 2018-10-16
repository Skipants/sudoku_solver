use std::fmt::Display;
use sudoku_cell_bitmask;

pub struct Board {
	pub answered_count: i32,
	pub candidates: [[i32; 9]; 9],
	pub fresh_coordinates: Vec<(usize, usize)>,
	pub values: [[i32; 9]; 9],
}

impl Board {
	pub fn new(raw_values: Vec<Vec<i32>>) -> Board {
		let mut answered_count = 0;
		let mut candidates = [[0x01FF; 9]; 9];
		let mut fresh_coordinates = vec![];
		let mut board_values = [[0; 9]; 9];

		for (i, row) in raw_values.into_iter().enumerate() {
			for (j, value) in row.into_iter().enumerate() {
				if value > 0 {
					answered_count += 1;
					board_values[i][j] = value;
					candidates[i][j] = sudoku_cell_bitmask::single_value_to_bitmask(value);
					fresh_coordinates.push((j, i));
				}
			}
		}

		Board {
			answered_count: answered_count,
			candidates: candidates,
			fresh_coordinates: fresh_coordinates,
			values: board_values,
		}
	}
}

impl Display for Board {
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
