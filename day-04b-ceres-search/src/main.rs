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

	pub fn x(&self, x: usize, y: usize) -> Vec<u8> {
		if !(1..=self.get_width() - 2).contains(&x) {
			panic!(
				"The x position must be between 1 and {}",
				self.get_width() - 2
			);
		}

		if !(1..=self.get_height() - 2).contains(&y) {
			panic!(
				"The y position must be between 1 and {}",
				self.get_height() - 2
			);
		}

		vec![
			self.data[y - 1][x - 1],
			self.data[y - 1][x + 1],
			self.data[y][x],
			self.data[y + 1][x - 1],
			self.data[y + 1][x + 1],
		]
	}

	pub fn get_width(&self) -> usize {
		self.width
	}

	pub fn get_height(&self) -> usize {
		self.height
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let grid = Grid::new(read_to_string("input.txt")?);

	let mut count = 0usize;
	for x in 1..=grid.get_width() - 2 {
		for y in 1..=grid.get_height() - 2 {
			if is_xmas(&grid.x(x, y)) {
				count += 1;
			}
		}
	}

	println!("{count}");

	Ok(())
}

fn is_xmas(chars: &[u8]) -> bool {
	// M.S
	// .A.
	// M.S
	const MATCH_MSAMS: &[u8] = b"MSAMS";
	// S.M
	// .A.
	// S.M
	const MATCH_SMASM: &[u8] = b"SMASM";
	// M.M
	// .A.
	// S.S
	const MATCH_MMASS: &[u8] = b"MMASS";
	// S.S
	// .A.
	// M.M
	const MATCH_SSAMM: &[u8] = b"SSAMM";

	matches!(chars, MATCH_MSAMS | MATCH_SMASM | MATCH_MMASS | MATCH_SSAMM)
}
