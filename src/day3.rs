// parse input of "claims" separated by newlines
// each claim is in the format of
// 		#123 @ 3,2: 5x4
// 		#id @ x,y: wxh

fn tokenise(input: &str) -> Vec<u32> {
	let mut result = Vec::new();
	let mut token_idx: i64 = -1;

	for (i, c) in input.char_indices() {
		let is_digit = c.is_digit(10);
		if token_idx != -1 {
			if !is_digit {
				// we've reached end of the token
				// println!("found token at {}..{}: {:?}", token_idx, i, &input[token_idx as usize..i]);
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
struct Rect {
	x: u32,
	y: u32,
}

impl Rect {
	fn new() -> Rect {
		Rect {
			x: 0,
			y: 0,
		}
	}
}

#[derive(Debug)]
struct Claim {
	id: u32,
	coords: Rect,
	size: Rect,
}

// impl Copy for Claim {}

// impl Clone for Claim {
// 	fn clone(&self) -> Claim {
// 		*self
// 	}
// }

impl Claim {
	fn new() -> Claim {
		Claim {
			id: 0,
			coords: Rect {
				x: 0,
				y: 0,
			},
			size: Rect {
				x: 0,
				y: 0,
			},
		}
	}

	fn parse(input: &str) -> Claim {
		let tokens = tokenise(input);
		// TODO check if tokens length is not exactly equal to what we need

		return Claim {
			id: tokens[0],
			coords: Rect {
				x: tokens[1],
				y: tokens[2],
			},
			size: Rect {
				x: tokens[3],
				y: tokens[4],
			},
		};
	}
}

// calculate how many square inches of the 1000x1000 square inch fabric have 2 or more overlapping claims
pub fn p1(input: &str) -> u32 {
	let mut result = 0;

	let mut fabric = Vec::new();
	fabric.resize(1000, [0_u32; 1000]);

	let claims = input.lines().map(|line| Claim::parse(line));

	for claim in claims {
		for x in 0..claim.size.x {
			let column = &mut fabric[(x + claim.coords.x) as usize];
			for y in 0..claim.size.y {
				let cell = &mut column[(y + claim.coords.y) as usize];
				*cell += 1;
				if *cell == 2 {
					result += 1;
				}
			}
		}
	}

	return result;
}

// find the single claim that does not overlap any other claim
pub fn p2(input: &str) -> u32 {
	let mut fabric = Vec::new();
	fabric.resize(1000, [None; 1000]);

	let claims: Vec<Claim> = input.lines().map(|line| Claim::parse(line)).collect();
	let mut intact_claims: Vec<u32> = claims.iter().map(|claim| claim.id).collect();

	for claim in claims {
		for x in 0..claim.size.x {
			let column = &mut fabric[(x + claim.coords.x) as usize];
			for y in 0..claim.size.y {
				let cell = column[(y + claim.coords.y) as usize];
				if let Some(id) = cell {
					let pos = intact_claims.iter().position(|c_id| *c_id == id);
					if let Some(idx) = pos {
						intact_claims.remove(idx);
					}

					let pos = intact_claims.iter().position(|c_id| *c_id == id);
					if let Some(idx) = pos {
						intact_claims.remove(idx);
					}
				} else {
					column[(y + claim.coords.y) as usize] = Some(claim.id);
				}
			}
		}
	}

	println!("claims len: {}", intact_claims.len());
	return intact_claims[0];
}
