#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::fmt::Display;

fn bitmask_to_single_value(bitmask: i32) -> i32 {
	match bitmask {
		0x001 => 1,
		0x002 => 2,
		0x004 => 3,
		0x008 => 4,
		0x010 => 5,
		0x020 => 6,
		0x040 => 7,
		0x080 => 8,
		0x100 => 9,
		_ => 0,
	}
}

struct Board {
	answered_count: i32,
	candidates: [[i32; 9]; 9],
	fresh_coordinates: Vec<(usize, usize)>,
	values: Vec<Vec<i32>>,
}

impl Board {
	pub fn new(raw_values: Vec<Vec<i32>>) -> Board {
		let mut answered_count = 0;
		let mut candidates = [[0x01FF; 9]; 9];
		let mut fresh_coordinates = vec![];
		let board_values = raw_values.into_iter().enumerate().map(|(i, row)| {
			row.into_iter().enumerate().map(|(j, value)| {
				if value > 0 {
					answered_count += 1;
					candidates[i][j] = 	match value {
						1 => 0x001,
						2 => 0x002,
						3 => 0x004,
						4 => 0x008,
						5 => 0x010,
						6 => 0x020,
						7 => 0x040,
						8 => 0x080,
						9 => 0x100,
						_ => 0x1FF,
					};
					fresh_coordinates.push((j, i));
				}

				value
			}).collect()
		}).collect();

		Board {
			answered_count: answered_count,
			candidates: candidates,
			fresh_coordinates: fresh_coordinates,
			values: board_values,
		}
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    for row in &self.values {
	    	for value in row {
	    		write!(f, "{}", value);
	    	}
	    	writeln!(f);
	    }

	    Ok(())
	}
}

fn fill_board_from_file(filename: String) -> Result<Board, std::io::Error> {
	let mut lines = BufReader::new(File::open(filename)?).lines();

    let values = (0..9).into_iter().map(|_i| {
    	let line_value: String = lines.next().unwrap().unwrap();
		line_value.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
	}).collect();

	Ok(Board::new(values))
}

fn solve_board(board: &mut Board) {
	while let Some((x, y)) = board.fresh_coordinates.pop() {
		if board.answered_count >= 81 { break }

		// Sole candidate strategies only work with already solved squares
		if board.values[y][x] != 0 {
			// Check sole candidate in row
			for i in 0..9 {
				if i == x // checking same square against itself
					|| board.values[y][i] != 0 // checking against already solved square
				{
						continue 
				}

				let prev_candidates = board.candidates[y][i];
				board.candidates[y][i] &= !board.candidates[y][x];

				let new_value = bitmask_to_single_value(board.candidates[y][i]);

				// If it's solved
				if (new_value > 0) {
					board.answered_count += 1;
					board.values[y][i] = new_value;
				}

				// If the candidates for that given block changed, then we want to check around it as well
				if board.candidates[y][i] != prev_candidates {
					board.fresh_coordinates.push((i, y))
				}
			}

			// Check sole candidate in column
			for i in 0..9 {
				if i == y || board.values[i][x] != 0 { continue }

				let prev_candidates = board.candidates[i][x];
				board.candidates[i][x] &= prev_candidates & !board.candidates[y][x];

				let new_value = bitmask_to_single_value(board.candidates[i][x]);

				// If it's solved
				if (new_value > 0) {
					board.answered_count += 1;
					board.values[i][x] = new_value;
				}

				// If the candidates for that given block changed, then we want to check around it as well
				if board.candidates[i][x] != prev_candidates {
					board.fresh_coordinates.push((x, i))
				}
			}
		}
	}
}

fn main() {
	let filename = env::args().nth(1).unwrap_or(String::from("test.txt"));
    let mut board = match fill_board_from_file(filename) {
    	Ok(board) => board,
    	Err(e) => {
    		println!("{:?}, {:?}", "Could not properly build board:", e);
    		std::process::exit(1);
    	}
    };

    solve_board(&mut board);

    println!("{}", board);
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
		solve_board(&mut board);

		assert_eq!(vec![
			vec![1, 2, 3, 4, 5, 6, 7, 8, 9,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0,],
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
		solve_board(&mut board);

		assert_eq!(vec![
			vec![3, 2, 1, 4, 5, 6, 7, 8, 9,],
			vec![0, 0, 2, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 3, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 4, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 5, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 6, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 7, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 8, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 9, 0, 0, 0, 0, 0, 0,],
		], board.values);
	}

	#[test]
	fn solves_hardest_puzzle() {
		let initial_values = vec![
			vec![8, 0, 0, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 3, 6, 0, 0, 0, 0, 0,],
			vec![0, 7, 0, 0, 9, 0, 2, 0, 0,],
			vec![0, 5, 0, 0, 0, 7, 0, 0, 0,],
			vec![0, 0, 0, 0, 4, 5, 7, 0, 0,],
			vec![0, 0, 0, 1, 0, 0, 0, 3, 0,],
			vec![0, 0, 1, 0, 0, 0, 0, 6, 8,],
			vec![0, 0, 8, 5, 0, 0, 0, 1, 0,],
			vec![0, 9, 0, 0, 0, 0, 4, 0, 0,],
		];
		let mut board = Board::new(initial_values);
		solve_board(&mut board);

		assert_eq!(vec![
			vec![8, 1, 2, 7, 5, 3, 6, 4, 9,],
			vec![9, 4, 3, 6, 8, 2, 1, 7, 5,],
			vec![6, 7, 5, 4, 9, 1, 2, 8, 3,],
			vec![1, 5, 4, 2, 3, 7, 8, 9, 6,],
			vec![3, 6, 9, 8, 4, 5, 7, 2, 1,],
			vec![2, 8, 7, 1, 6, 9, 5, 3, 4,],
			vec![5, 2, 1, 9, 7, 4, 3, 6, 8,],
			vec![4, 3, 8, 5, 2, 6, 9, 1, 7,],
			vec![7, 9, 6, 3, 1, 8, 4, 5, 2,],
		], board.values);
	}
}
