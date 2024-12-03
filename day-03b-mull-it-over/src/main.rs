use std::{error::Error, fs::read_to_string, ops::Range};

fn main() -> Result<(), Box<dyn Error>> {
	let text = read_to_string("input.txt")?;
	let result = extract_multiplications(&text);

	println!("{result}");

	Ok(())
}

fn extract_non_availability(chars: &str) -> Vec<Range<usize>> {
	const KEYWORD_DO: &str = "do()";
	const KEYWORD_DO_NOT: &str = "don't()";

	let mut index = 0usize;
	let mut ranges = vec![];

	while let Some(start) = chars[index..].find(KEYWORD_DO_NOT) {
		let start = index + start + KEYWORD_DO_NOT.len();

		if let Some(end) = chars[start..].find(KEYWORD_DO) {
			let end = start + end + KEYWORD_DO.len();

			ranges.push(start..end);
			index = end;
		} else {
			ranges.push(start..usize::MAX);
			break;
		}
	}

	println!("{ranges:?}");

	ranges
}

fn extract_multiplications(chars: &str) -> i64 {
	let non_availability_regions = extract_non_availability(chars);

	let mut accumulation = 0i64;
	let mut index = 0usize;
	while let Some((value, updated_index)) =
		extract_next_multiplication(chars, index)
	{
		if !non_availability_regions
			.iter()
			.any(|range| range.contains(&updated_index))
		{
			accumulation += value;
		}

		index = updated_index;
	}

	accumulation
}

fn extract_next_multiplication(
	chars: &str,
	index: usize,
) -> Option<(i64, usize)> {
	const PREFIX: &str = "mul(";

	if let Some(find_index) = chars[index..].find(PREFIX) {
		let mut current_index = index + find_index + PREFIX.len();

		let a: i64;
		let b: i64;
		if let Some((value, updated_index)) =
			extract_number(chars, current_index, ',')
		{
			a = value;
			current_index = updated_index;
		} else {
			return extract_next_multiplication(
				chars,
				index + find_index + PREFIX.len(),
			);
		}

		if let Some((value, updated_index)) =
			extract_number(chars, current_index, ')')
		{
			b = value;
			current_index = updated_index;
		} else {
			return extract_next_multiplication(
				chars,
				index + find_index + PREFIX.len(),
			);
		}

		return Some((a * b, current_index));
	}

	None
}

fn extract_number(
	chars: &str,
	index: usize,
	delimiter: char,
) -> Option<(i64, usize)> {
	let mut buffer = String::with_capacity(3);

	for value in chars[index..].chars() {
		if value.is_ascii_digit() {
			if buffer.len() == 3 {
				return None;
			}

			buffer += value.to_string().as_str();
		} else if value == delimiter && !buffer.is_empty() {
			return Some((
				buffer.parse::<i64>().unwrap(),
				index + buffer.len() + 1,
			));
		} else {
			return None;
		}
	}

	None
}
