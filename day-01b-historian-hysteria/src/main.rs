use std::{collections::HashMap, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
	let mut list: Vec<i32> = Vec::new();
	let mut map: HashMap<i32, i32> = HashMap::new();

	for mut line in read_to_string("input.txt")?
		.split("\n")
		.filter(|line| !line.is_empty())
		.map(|line| line.split("   "))
	{
		list.push(line.next().unwrap().parse::<i32>()?);

		let value = line.next().unwrap().parse::<i32>()?;
		map.insert(value, map.get(&value).map_or(1, |v| v + 1));
	}

	let difference: i32 = list //
		.into_iter()
		.map(|value| match map.get(&value) {
			Some(occurrences) => value * occurrences,
			None => 0,
		})
		.sum();
	println!("The difference is {difference}");

	Ok(())
}
