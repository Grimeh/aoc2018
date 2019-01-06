// parse input of "claims" separated by newlines
// each claim is in the format of
// 		#123 @ 3,2: 5x4
// 		#id @ x,y: wxh

use std::error::Error;
use common::Result;

fn tokenise(input: &str) -> Vec<u32> {
	let mut result = Vec::new();
	let mut token_idx: i64 = -1;

	for (i, c) in input.char_indices() {
		let is_digit = c.is_digit(10);
		if token_idx != -1 {
			if !is_digit {
				// we've reached end of the token
				result.push(input[token_idx as usize..i].parse::<u32>().unwrap());
				token_idx = -1;
			}
		} else {
			if is_digit {
				// we've found a new token
				token_idx = i as i64;
			}
		}
	}

	if token_idx != -1 {
		result.push(input[token_idx as usize..].parse::<u32>().unwrap());
	}

	return result;
}

#[derive(Debug)]
struct Claim {
	id: u32,
	x: u32,
	y: u32,
	w: u32,
	h: u32,
}

impl Claim {
	// dumb parser, doesn't check for correct syntax, just number of numbers
	fn parse(input: &str) -> Result<Claim> {
		let tokens = tokenise(input);

		if tokens.len() != 5 {
			return err!("invalid input");
		}

		Ok(Claim {
			id: tokens[0],
			x: tokens[1],
			y: tokens[2],
			w: tokens[3],
			h: tokens[4],
		})
	}

	fn points(&self) -> Points {
		Points::new(self)
	}
}

// iterator impl shamelessly stolen (w/ slight modification) from BurntSushi's impl
// @ https://github.com/BurntSushi/advent-of-code/blob/master/aoc03/src/main.rs

// lifetime required for claim reference
struct Points<'a> {
	claim: &'a Claim,
	i: u32,
}

impl<'a> Points<'a> {
	fn new(claim: &'a Claim) -> Points<'a> {
		Points {
			claim: claim,
			i: 0,
		}
	}
}

impl<'a> Iterator for Points<'a> {
	type Item = (u32, u32);

	fn next(&mut self) -> Option<(u32, u32)> {
		let x = self.i / self.claim.h;
		if x > self.claim.w {
			return None;
		}
		let result = (self.claim.x + x, self.claim.y + (self.i % self.claim.h));
		self.i += 1;
		Some(result)
	}
}

fn parse_input(input: &str) -> Result<Vec<Claim>> {
	let mut claims = Vec::new();

	for line in input.lines() {
		let claim = Claim::parse(line).or_else(|_| {
			err!("could not parse line \"{}\"", line)
		})?;
		claims.push(claim);
	}

	return Ok(claims);
}

/* calculate how many square inches of the 1000x1000 square inch fabric
   have 2 or more overlapping claims */
pub fn p1(input: &str) -> Result<u32> {
	let mut result = 0;

	let mut fabric = Vec::new();
	fabric.resize(1000, [0_u32; 1000]);
	let claims = parse_input(input)?;

	for claim in claims {
		for (x, y) in claim.points() {
			let cell = &mut fabric[x as usize][y as usize];
			*cell += 1;
			if *cell == 2 {
				result += 1;
			}
		}
	}

	Ok(result)
}

// same as above except with a hashmap, for benchmarking
pub fn p1_hashmap(input: &str) -> Result<u32> {
	use hashbrown::HashMap;

	let mut result = 0;
	// exact number of cells actually used is 350,883
	let mut fabric = HashMap::with_capacity(350883);
	let claims = parse_input(input)?;

	for claim in claims {
		for (x, y) in claim.points() {
			let cell = fabric.entry((x, y)).or_insert(0);
			*cell += 1;
			if *cell == 2 {
				result += 1;
			}
		}
	}

	Ok(result)
}

// find the single claim that does not overlap any other claim
pub fn p2(input: &str) -> Result<u32> {
	let mut fabric = Vec::new();
	fabric.resize(1000, [None; 1000]);

	let claims = parse_input(input)?;
	// this is sorted by default; input is sorted already
	let mut intact_claims: Vec<u32> = claims.iter().map(|claim| claim.id).collect();

	for claim in claims {
		for (x, y) in claim.points() {
			let cell = fabric[x as usize][y as usize];
			if let Some(id) = cell {
				let pos = intact_claims.binary_search(&claim.id);
				if let Ok(idx) = pos {
					intact_claims.remove(idx);
				}

				let pos = intact_claims.binary_search(&id);
				if let Ok(idx) = pos {
					intact_claims.remove(idx);
				}
			} else {
				fabric[x as usize][y as usize] = Some(claim.id);
			}
		}
	}

	Ok(intact_claims[0])
}

pub fn p2_hashmap(input: &str) -> Result<u32> {
	use hashbrown::HashSet;

	let mut fabric = Vec::new();
	fabric.resize(1000, [None; 1000]);

	let claims = parse_input(input)?;

	let mut intact_claims = HashSet::with_capacity(claims.len());
	for claim in claims.iter() {
		intact_claims.insert(claim.id);
	}

	for claim in claims {
		for (x, y) in claim.points() {
			let cell = fabric[x as usize][y as usize];
			if let Some(id) = cell {
				intact_claims.remove(&claim.id);
				intact_claims.remove(&id);
			} else {
				fabric[x as usize][y as usize] = Some(claim.id);
			}
		}
	}

	match intact_claims.iter().next() {
		Some(value) => Ok(*value),
		None => err!("could not find a non-overlapping claim"),
	}
}
