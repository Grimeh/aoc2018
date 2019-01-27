use common::Result;
use std::string::String;
use std::collections::HashMap;

fn reacts(a: u8, b: u8) -> bool {
	if a > b {
		a - b == 32
	} else {
		b - a == 32
	}
}

fn collapse_vec(input: &str) -> String {
	if input.len() == 0 {
		return input.to_string();
	}

	let string = input.as_bytes().to_vec();
	let mut result = Vec::new();
	result.reserve(string.len());
	let mut idx = 0;
	while idx + 1 < string.len() {
		let a = string[idx];
		let b = string[idx + 1];
		if reacts(a, b) {
			idx += 2;
		} else {
			result.push(a);
			idx += 1;
		}
	}

	if idx < string.len() {
		result.push(string[idx]);
	}

	String::from_utf8(result).unwrap()
}

fn fully_collapse(input: &str) -> String {
	let mut string = input.to_string();
	let mut length = string.len();
	loop {
		string = collapse_vec(&string);
		if length == string.len() {
			// no more collapsing to be done
			break;
		}
		length = string.len();
	}
	return string;
}

pub fn p1(input: &str) -> usize {
	let polymer = fully_collapse(input);
	return polymer.len();
}

pub fn p2(input: &str) -> usize {
	let mut best = std::usize::MAX;
	for c in b'a'..b'z' {
		let filtered: String = input.chars().filter(|el| el.to_ascii_lowercase() != c as char).collect();
		let collapsed = fully_collapse(filtered.as_str());
		if collapsed.len() < best {
			best = collapsed.len();
		}
	}

	return best;
}