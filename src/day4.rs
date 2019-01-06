use std::collections::HashMap;
use std::error::Error;

use common::Result;

// Some assumptions have been made here based on the input data,
// - the year never changes, so we ignore it
// - we don't need to worry about how many days in a month
// - the time is always between [23.46-00.59]
// We reduce the input date and time to "minutes to/from midnight on this day of the year",
// so      [1518-11-21 23:56] = 4 minutes to midnight on the 351st day of the year
// and     [1518-11-22 00:13] = 13 minutes from midnight on the 351st day of the year
// Note that these refer to the same "day of the year" despite falling on subsequent days

// sample data
/*
[1518-11-21 23:56] Guard #157 begins shift
[1518-11-22 00:13] falls asleep
[1518-11-22 00:44] wakes up
*/

#[derive(Debug)]
struct GuardRecord {
	day: u32, // day of the year
	time: i32, // minutes from midnight
	awake: bool, // is the guard currently awake?
}

impl GuardRecord {
	fn parse(input: &str) -> Option<GuardRecord> {
		let mut tokens = input
			.split(|c| c == '[' || c == '-' || c == ' ' || c == ']' || c == ':')
			.filter(|token| !token.is_empty());

		let _ = tokens.next()?; // year

		let month = tokens
			.next()?
			.parse::<u32>();
		if month.is_err() {
			return None
		}
		let month = month.unwrap();

		let day = tokens
			.next()?
			.parse::<u32>();
		if day.is_err() {
			return None;
		}
		let day = day.unwrap();

		let hour = tokens.next()?;

		let minute = tokens
			.next()?
			.parse::<i32>();
		if minute.is_err() {
			return None
		}
		let mut minute = minute.unwrap();

		if hour != "00" {
			minute -= 60;
		}


		Some(GuardRecord {
			day: ((month * 30) + day),
			time: minute,
			awake: input.contains("begins") || input.contains("wakes"),
		})
	}
}

fn find_guard_id(input: &str) -> Option<u32> {
	let start_idx = input.find('#')?;
	let slice = &input[start_idx + 1..];
	let end_idx = slice.find(' ').unwrap(); // invalid input if not found
	let slice = &slice[..end_idx];

	Some(slice.parse::<u32>().unwrap()) // invalid input if not able to convert to u16
}

pub fn p1(input: &str) -> Result<u32> {
	let mut records: HashMap<u32, Vec<GuardRecord>>= HashMap::new();

	let mut lines: Vec<&str> = input.lines().collect();
	lines.sort();

	let mut cur_id = 0;
	for line in lines.iter() {
//		println!("{}", line);
		let record = GuardRecord::parse(line);
		if record.is_none() {
			return err!("Could not parse input")
		}
		let record = record.unwrap();

		let record_id = find_guard_id(line);
		if let Some(id) = record_id {
			cur_id = id;
		}

		let guard_records = records.entry(cur_id).or_insert(Vec::new());
		guard_records.push(record);
	}

	let mut most_asleep = (0, 0);
	for (id, guard_records) in &records {
		let mut asleep = None;
		let mut time_asleep = 0;
		for record in guard_records {
			if !record.awake {
				asleep = Some(record);
			} else if asleep.is_some() {
				time_asleep += (record.time - asleep.unwrap().time) as u32;
				asleep = None;
			}
		}

		if time_asleep > most_asleep.1 {
			most_asleep = (*id, time_asleep);
		}
	}

	let mut sleep_pattern = [0; 120];
	let guard_records = records.get(&most_asleep.0).unwrap();
	let mut time_asleep = None;
	for record in guard_records {
		if !record.awake {
			time_asleep = Some(record.time);
		} else {
			if time_asleep.is_some() {
				let time_asleep = time_asleep.unwrap();
				let duration = record.time - time_asleep;
				let range = (time_asleep + 60) as usize..(record.time + 60) as usize;
				let minutes = &mut sleep_pattern[range];
				for mut minute in minutes {
					*minute += duration;
				}
			}
			time_asleep = None;
		}
	}


	let mut best_minute = (0, 0);
	for (idx, duration) in sleep_pattern.iter().enumerate() {
//		println!("minute {}: {}", idx, duration);
		if duration > &best_minute.0 {
			best_minute = (*duration, idx);
		}
	}

	return Ok(((best_minute.0 - 60) * best_minute.1 as i32) as u32);
}