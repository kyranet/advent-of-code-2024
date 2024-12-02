#![feature(iter_map_windows)]
#![feature(iter_chain)]

use std::{cmp::Ordering, error::Error, fs::read_to_string, iter::chain};

fn main() -> Result<(), Box<dyn Error>> {
	let safe = read_to_string("input.txt")?
		.split("\n")
		.filter(|line| !line.is_empty())
		.map(|line| {
			is_safe(
				&mut line
					.split_whitespace()
					.map(|part| part.parse::<i32>().unwrap()),
			)
		})
		.filter(|result| *result)
		.count();

	println!("{safe}");

	Ok(())
}

fn is_safe(line: &mut impl Iterator<Item = i32>) -> bool {
	let a = line.next().unwrap();
	let b = line.next().unwrap();
	let order = b.cmp(&a);

	chain(vec![a, b], line)
		.map_windows(|[a, b]| is_safe_pair(order, *a, *b))
		.min()
		.unwrap()
}

fn is_safe_pair(order: Ordering, a: i32, b: i32) -> bool {
	match order {
		Ordering::Equal => false,
		Ordering::Greater => b > a && (1..=3).contains(&(b - a)),
		Ordering::Less => a > b && (1..=3).contains(&(a - b)),
	}
}
