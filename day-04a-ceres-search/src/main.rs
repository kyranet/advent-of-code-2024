use std::{error::Error, fs::read_to_string};

struct Grid {
	width: usize,
	height: usize,
	data: Vec<Vec<u8>>,
}

impl Grid {
	pub fn new(string: String) -> Self {
		let data: Vec<Vec<u8>> =
			string.lines().map(|line| line.as_bytes().to_vec()).collect();
		let width = data[0].len();
		let height = data.len();

		Self { width, height, data }
	}

	pub fn row(&self, index: usize) -> Vec<u8> {
		if index >= self.get_height() {
			panic!("{index} should be lower than {}", self.get_height());
		}

		self.data.iter().map(|line| line[index]).collect()
	}

	pub fn column(&self, index: usize) -> Vec<u8> {
		if index >= self.get_width() {
			panic!("{index} should be lower than {}", self.get_width());
		}

		self.data[index].clone()
	}

	pub fn left_diagonal(&self, index: usize) -> Vec<u8> {
		if index >= self.get_diagonals() {
			panic!("{index} should be lower than {}", self.get_diagonals());
		}

		let mut x: usize;
		let mut y: usize;
		if index < self.get_height() {
			x = 0;
			y = self.get_height() - index - 1;
		} else {
			x = index - self.get_height() + 1;
			y = 0;
		}

		// print!("LEFT_DIAGONAL {index:2}: [{x},{y}] ");

		let mut output = vec![];
		while x < self.get_width() && y < self.get_height() {
			output.push(self.data[y][x]);
			x += 1;
			y += 1;
		}

		// println!("{:?}", String::from_utf8(output.clone()).unwrap());

		output
	}

	pub fn right_diagonal(&self, index: usize) -> Vec<u8> {
		if index >= self.get_diagonals() {
			panic!("{index} should be lower than {}", self.get_diagonals());
		}

		let mut x: usize;
		let mut y: usize;
		if index < self.get_height() {
			x = self.get_width() - 1;
			y = self.get_height() - index - 1;
		} else {
			x = (self.get_width() - 2) - (index - self.get_height());
			y = 0;
		}

		let mut output = vec![];
		while y < self.get_height() {
			output.push(self.data[y][x]);

			match x.checked_sub(1) {
				Some(value) => x = value,
				None => break,
			}

			y += 1;
		}

		output
	}

	pub fn get_width(&self) -> usize {
		self.width
	}

	pub fn get_height(&self) -> usize {
		self.height
	}

	pub fn get_diagonals(&self) -> usize {
		self.get_width() + self.get_height() - 1
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let grid = Grid::new(read_to_string("input.txt")?);

	let mut count = 0usize;
	for i in 0..grid.get_width() {
		count += count_occurrences(&grid.column(i));
	}

	for i in 0..grid.get_height() {
		count += count_occurrences(&grid.row(i));
	}

	// Run from 4 to n - 4, since the ranges (0..3) and (n-3..n) will yield
	// vectors with a size of 3 or less:
	for i in 4..=(grid.get_diagonals() - 4) {
		count += count_occurrences(&grid.left_diagonal(i));
		count += count_occurrences(&grid.right_diagonal(i));
	}

	println!("{count}");

	Ok(())
}

fn count_occurrences(chars: &[u8]) -> usize {
	chars
		.windows(4)
		.fold(0, |acc, chars| if is_xmas(chars) { acc + 1 } else { acc })
}

fn is_xmas(chars: &[u8]) -> bool {
	matches!(chars, b"XMAS" | b"SAMX")
}
