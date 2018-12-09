// input is a list of "frequency" changes (eg. +1, -2) sepearted by newlines

use std::num::ParseIntError;

// return the final value
pub fn d1p1(input: &str) -> Result<i64, ParseIntError> {
	let mut frequency = 0;

	for line in input.lines() {
		frequency += line.parse::<i64>()?;
	}

	return Ok(frequency);
}

// return the first frequency value encountered more than once
pub fn d1p2(input: &str) -> Result<i64, ParseIntError> {
	use std::collections::HashSet;

	let mut frequency = 0;
	let mut occurences = HashSet::new();

	// loop until we encounter a frequency for the second time
	loop {
		for line in input.lines() {
			frequency += line.parse::<i64>()?;
			match occurences.get(&frequency) {
				Some(&f) => return Ok(f),
				None => occurences.insert(frequency),
			};
		}
	}
}