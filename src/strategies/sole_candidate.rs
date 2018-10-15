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
				0...2 => (0..=2),
				3...5 => (3..=5),
				6...8 => (6..=8),
				_ => panic!("Expected coord value to be within 0-8, but was {:?}", coord),	
			}
		};

		for i in compute_block_range(y) {
			for j in compute_block_range(x) {
				sole_candidate_comparison(board, x, y, j, i);
			}
		}
	}
}
