// input is a list of box IDs (combination of english alphabet letters) separated by newlines
// eg. abcdef, aabbcc, ababab, etc.

// calculate a "checksum" of all the box IDs
pub fn p1(input: &str) -> u64 {
	let offset = 'a' as usize;

	println!("offsets: a {}, b {}", 'a' as usize - offset, 'b' as usize - offset);

	let mut two_count = 0;
	let mut three_count = 0;

	// scales at (n * (2n))
	for line in input.lines() {
		let mut charvec: Vec<u8> = Vec::new();
		charvec.resize(26, 0); // the english alphabet

		// count occurences of each letter
		for c in line.chars() {
			charvec[c as usize - offset] += 1;
		}

		// increment two_count and three_count as appropriate
		let mut have_incremented_two = false;
		let mut have_incremented_three = false;
		for count in charvec.iter() {
			if *count == 2 && !have_incremented_two {
				two_count += 1;
				have_incremented_two = true;
			} else if *count == 3 && !have_incremented_three {
				three_count += 1;
				have_incremented_three = true;
			}

			if have_incremented_two && have_incremented_three {
				break;
			}
		}
	}

	return two_count * three_count;
}

fn check_ids(a: &str, b: &str) -> String {
	let mut result = String::new();

	let achars = a.chars();
	let bchars = b.chars();

	for (c, b) in achars.zip(bchars) {
		if c == b {
			result.push(c);
		}
	}

	return result;
}

// find the two boxes with only one differing character (location sensitive),
// return the common characters
pub fn p2(input: &str) -> Option<String> {
	let lines: Vec<&str> = input.lines().collect();
	let mut a = 0;
	while a < lines.len() {
		let ida = lines[a];
		let mut b = a + 1;
		while b < lines.len() {
			let idb = lines[b];
			let matching = check_ids(ida, idb);
			if matching.len() == ida.len() - 1 {
				return Some(matching);
			}
			b += 1;
		}
		a += 1;
	}

	return None;
}