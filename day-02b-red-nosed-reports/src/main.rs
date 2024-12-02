#![feature(iter_map_windows)]

use std::{cmp::Ordering, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
	let safe = read_to_string("input.txt")?
		.split("\n")
		.filter(|line| !line.is_empty())
		.map(|line| {
			is_safe_dampened(
				line.split_whitespace()
					.map(|part| part.parse::<i32>().unwrap())
					.collect::<Vec<i32>>()
					.as_slice(),
			)
		})
		.filter(|result| *result)
		.count();

	println!("{safe}");

	Ok(())
}

fn is_safe_dampened(line: &[i32]) -> bool {
	if is_safe(line.to_vec()) {
		return true;
	}

	(0..line.len())
		.map(|i| {
			is_safe({
				let mut vec = Vec::with_capacity(line.len() - 1);
				vec.extend_from_slice(&line[..i]);
				vec.extend_from_slice(&line[(i + 1)..]);
				vec
			})
		})
		.max()
		.unwrap()
}

fn is_safe(line: Vec<i32>) -> bool {
	let order = line[1].cmp(&line[0]);
	let result = line
		.windows(2)
		.map(|pair| is_safe_pair(order, pair[0], pair[1]))
		.min()
		.unwrap();

	result
}

fn is_safe_pair(order: Ordering, a: i32, b: i32) -> bool {
	match order {
		Ordering::Equal => false,
		Ordering::Greater => b > a && (1..=3).contains(&(b - a)),
		Ordering::Less => a > b && (1..=3).contains(&(a - b)),
	}
}
