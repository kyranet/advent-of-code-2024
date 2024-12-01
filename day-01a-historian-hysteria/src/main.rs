use std::{error::Error, fs::read_to_string, iter::zip};

fn main() -> Result<(), Box<dyn Error>> {
	let mut list_l: Vec<i32> = Vec::new();
	let mut list_r: Vec<i32> = Vec::new();

	for mut line in read_to_string("input.txt")?
		.split("\n")
		.filter(|line| !line.is_empty())
		.map(|line| line.split("   "))
	{
		list_l.push(line.next().unwrap().parse::<i32>()?);
		list_r.push(line.next().unwrap().parse::<i32>()?);
	}

	list_l.sort();
	list_r.sort();

	let difference: i32 = zip(&list_l, &list_r) //
		.map(|(l, r)| (l - r).abs())
		.sum();
	println!("The difference is {difference}");

	Ok(())
}
