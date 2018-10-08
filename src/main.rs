#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{ self, BufReader, BufRead };
use std::fmt::Display;

struct Coord {
	x: usize,
	y: usize,
}

struct Board {
	answered_count: u8,
	fresh_coordinates: Vec<Coord>,
	rows: Vec<Vec<u8>>,
}

impl Board {
	fn fill_from_file(&mut self, filename: String) -> io::Result<()> {
		let f = BufReader::new(File::open(filename)?);

	    for (i, line) in f.lines().enumerate() {
	    	let line_value = line.unwrap();

	    	if line_value.is_empty() { continue; }

	    	let numbers = vec![];

	    	for (j, num) in line_value
	    		.split_whitespace()
	    		.map(|x| x.parse::<u8>().unwrap())
	    		.enumerate() {

	    		if (num != 0) {
	    			&self.fresh_coordinates.push(Coord{ x: i, y: j });
	    		}

	    		numbers.push(num)
    		}

    		&self.rows.push(numbers);
    	}

		Ok(())
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    for row in &self.rows {
	    	for num in row {
	    		write!(f, "{}", num);
	    	}
	    	writeln!(f);
	    }

	    Ok(())
	}
 }

fn main() {
	let mut board = Board { rows: vec![] };
  	let filename = env::args().nth(1).unwrap_or(String::from("test.txt"));
    
    board.fill_from_file(filename);



    println!("{}", board);
}
