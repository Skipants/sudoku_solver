use board::Board;
use sudoku_cell_bitmask;

pub fn solve(board: &mut Board) {
	for y in (0..9) {
		for x in (0..9) {

			if board.values[y][x] != 0 { continue }

			let bitmask = board.candidates[y][x];
			let value = sudoku_cell_bitmask::bitmask_to_single_value(bitmask);

			if value != 0 {
				board.values[y][x] = value;
				board.answered_count += 1;
				board.fresh_coordinates.push((x, y));
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn solves_unique_candidate() {
		let initial_values = vec![
			vec![0, 0, 4, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 4, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![5, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 4, 0, 0, 0,],
		];
		let mut board = Board::new(initial_values);
		board.candidates[7][0] = 0x08;

		solve(&mut board);

		assert_eq!([
			[0, 0, 4, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 4, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 0, 0, 0, 0,],
			[5, 0, 0, 0, 0, 0, 0, 0, 0,],
			[4, 0, 0, 0, 0, 0, 0, 0, 0,],
			[0, 0, 0, 0, 0, 4, 0, 0, 0,],
		], board.values);
	}
}
