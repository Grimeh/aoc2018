// input is a list of "frequency" changes (eg. +1, -2) sepearted by newlines

// return the final value
pub fn d1p1(input: &str) -> i64 {
	let mut frequency = 0;

	for line in input.lines() {
		frequency += line.parse::<i64>().unwrap();
	}

	return frequency;
}

// return the first frequency value encountered more than once
pub fn d1p2(input: &str) -> i64 {
	use std::collections::HashSet;

	let mut frequency = 0;
	let mut occurences = HashSet::new();

	// loop until we encounter a frequency for the second time
	while true {
		for line in input.lines() {
			frequency += line.parse::<i64>().unwrap();
			match occurences.get(&frequency) {
				Some(&f) => return f,
				None => occurences.insert(frequency),
			};
		}
	}

	panic!("Something went wrong");
}