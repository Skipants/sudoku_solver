use board::Board;
use sudoku_cell_bitmask;

fn sole_candidate_comparison(board: &mut Board, solved_x: usize, solved_y: usize, candidate_x: usize, candidate_y: usize) {
	if solved_x == candidate_x && solved_y == candidate_y || board.values[candidate_y][candidate_x] > 0 {
		return
	}

	let prev_candidates = board.candidates[candidate_y][candidate_x];

	board.candidates[candidate_y][candidate_x] &= !sudoku_cell_bitmask::single_value_to_bitmask(board.values[solved_y][solved_x]);

	let new_value = sudoku_cell_bitmask::bitmask_to_single_value(board.candidates[candidate_y][candidate_x]);

	// If it's solved
	if (new_value > 0) {
		board.answered_count += 1;
		board.values[candidate_y][candidate_x] = new_value;
	}

	// If the candidates for that given cell changed, then we want to check around it as well
	if board.candidates[candidate_y][candidate_x] != prev_candidates {
		if (!board.fresh_coordinates.contains(&(candidate_x, candidate_y))) {
			board.fresh_coordinates.push((candidate_x, candidate_y))
		}
	}	
}

pub fn solve(board: &mut Board, x: usize, y: usize) {
	// Sole candidate strategies only work with already solved squares
	if board.values[y][x] != 0 {
		// Check sole candidates in row and column
		for i in 0..9 {
			sole_candidate_comparison(board, x, y, x, i);
			sole_candidate_comparison(board, x, y, i, y);
		}

		// Check sole candidates in block
		let compute_block_range = |coord: usize| {
			match coord {
				0...2 => (0..3),
				3...5 => (3..6),
				6...8 => (6..9),
				_ => panic!("Expected coord value to be within 0-8, but was {:?}", coord),	
			}
		};

		for i in compute_block_range(y) {
			for j in compute_block_range(x) {
				if y == 3 && x == 4 { println!("value: {}, {} {}", board.values[y][x], j, i); }
				sole_candidate_comparison(board, x, y, j, i);
				if y == 3 && x == 4 { println!("candidate: {:x}", board.candidates[j][i]); }
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn solves_sole_candidate_row() {
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

		for i in 0..9 {
			solve(&mut board, i, 0);
		}

		assert_eq!([
			[1, 2, 3, 4, 5, 6, 7, 8, 9,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
		], board.values);
	}

	#[test]
	fn solves_sole_candidate_col() {
		let initial_values = vec![
			vec![3, 2, 1, 4, 5, 6, 7, 8, 9,],
			vec![0, 0, 2, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 3, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 4, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 5, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 7, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 8, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 9, 0, 0, 0, 0, 0, 0,],
		];
		let mut board = Board::new(initial_values);

		for i in 0..9 {
			solve(&mut board, 2, i);
		}

		assert_eq!([
			[3, 2, 1, 4, 5, 6, 7, 8, 9,],
			[0, 0, 2, 0, 0, 0, 0, 0, 0,],
			[0, 0, 3, 0, 0, 0, 0, 0, 0,],
			[0, 0, 4, 0, 0, 0, 0, 0, 0,],
			[0, 0, 5, 0, 0, 0, 0, 0, 0,],
			[0, 0, 6, 0, 0, 0, 0, 0, 0,],
			[0, 0, 7, 0, 0, 0, 0, 0, 0,],
			[0, 0, 8, 0, 0, 0, 0, 0, 0,],
			[0, 0, 9, 0, 0, 0, 0, 0, 0,],
		], board.values);
	}

	#[test]
	fn solves_sole_candidate_block() {
		let initial_values = vec![
			vec![3, 2, 1, 4, 5, 6, 7, 8, 9,],
			vec![0, 0, 2, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 3, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 4, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 5, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 6, 0, 0, 0, 0, 0, 0,],
			vec![0, 5, 7, 0, 0, 0, 0, 0, 0,],
			vec![1, 6, 8, 0, 0, 0, 0, 0, 0,],
			vec![2, 3, 9, 0, 0, 0, 0, 0, 0,],
		];
		let mut board = Board::new(initial_values);

		solve(&mut board, 0, 7);
		solve(&mut board, 0, 8);
		solve(&mut board, 1, 6);
		solve(&mut board, 1, 7);
		solve(&mut board, 1, 8);
		solve(&mut board, 2, 6);
		solve(&mut board, 2, 7);
		solve(&mut board, 2, 8);

		assert_eq!([
			[3, 2, 1, 4, 5, 6, 7, 8, 9,],
			[0, 0, 2, 0, 0, 0, 0, 0, 0,],
			[0, 0, 3, 0, 0, 0, 0, 0, 0,],
			[0, 0, 4, 0, 0, 0, 0, 0, 0,],
			[0, 0, 5, 0, 0, 0, 0, 0, 0,],
			[0, 0, 6, 0, 0, 0, 0, 0, 0,],
			[4, 5, 7, 0, 0, 0, 0, 0, 0,],
			[1, 6, 8, 0, 0, 0, 0, 0, 0,],
			[2, 3, 9, 0, 0, 0, 0, 0, 0,],
		], board.values);
	}
}
