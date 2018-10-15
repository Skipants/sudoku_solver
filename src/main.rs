#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::fmt::Display;

mod sudoku_cell_bitmask;
mod board;
mod strategies;

use board::Board;

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
		strategies::sole_candidate::solve(board, x, y);
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
		solve_board(&mut board);

		assert_eq!(vec![
			vec![3, 2, 1, 4, 5, 6, 7, 8, 9,],
			vec![0, 0, 2, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 3, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 4, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 5, 0, 0, 0, 0, 0, 0,],
			vec![0, 0, 6, 0, 0, 0, 0, 0, 0,],
			vec![4, 5, 7, 0, 0, 0, 0, 0, 0,],
			vec![1, 6, 8, 0, 0, 0, 0, 0, 0,],
			vec![2, 3, 9, 0, 0, 0, 0, 0, 0,],
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
