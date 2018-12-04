// parse input of "claims" separated by newlines
// each claim is in the format of
// 		#123 @ 3,2: 5x4
// 		#id @ x,y: wxh

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

impl Claim {
	fn new() -> Claim {
		Claim {
			id: 0,
			coords: Rect {
				x: 0,
				y: 0,
			},
			size: Rect {
				w: 0,
				h: 0,
			},
		}
	}

	fn parse(input: &str) -> Claim {
		let mut claim = Claim {
			id: 0,
			coords: Rect {
				x: 0,
				y: 0,
			},
			size: Rect {
				w: 0,
				h: 0,
			},
		};

		let chars = line.chars();

		// extract ID
		chars.next(); // '#'
		let idStr = chars
			.by_ref() // do not consume the iterator
			.take_while(|c| **c != ' ') // take the number following the '#'
			.as_str();
		claim.id = idStr.parse::<u32>().unwrap();

		chars.next(); // '@'
		chars.next(); // ' '

		// extract coords of rect
		claim.coords.x = chars.next().unwrap().to_digit(10).unwrap();
		chars.next(); // ','
		claim.coords.y = chars.next().unwrap().to_digit(10).unwrap();

		// extract size of rect
		chars.next(); // ':'
		chars.next(); // ' '
		claim.size.x = chars.next().unwrap().to_digit(10).unwrap();
		chars.next(); // 'x'
		claim.size.y = chars.next().unwrap().to_digit(10).unwrap();

		return claim;
	}
}

// calculate how many square inches of the 1000x1000 square inch fabric have 2 or more overlapping claims
fn p1(input: &str) -> u32 {
	let claims = input.lines().map(|line| Claim::parse(line));
}