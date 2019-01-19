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

//#[derive(Debug)]
//struct GuardRecord {
//	day: u32, // day of the year
//	time: i32, // minutes from midnight
//	awake: bool, // is the guard currently awake?
//}

fn parse_from_str<T>(input: &str) -> Option<T>
	where T: std::str::FromStr,
		  <T as std::str::FromStr>::Err : std::fmt::Debug
{
	let value = input
		.parse::<T>();
	if value.is_err() {
		return None;
	}
	return Some(value.unwrap());
}

#[derive(Debug, PartialEq)]
enum InputEvent {
	BeginShift(u32),
	FallAsleep,
	Wake,
}

#[derive(Debug)]
struct Input {
	day: u32,
	minute: i32,
	event: InputEvent,
}

impl Input {
	fn is_shift_start(&self) -> bool {
		use std::mem::discriminant;
		discriminant(&self.event) == discriminant(&InputEvent::BeginShift(0))
	}

	fn from_str(input: &str) -> Result<Input> {
		let tokens: Vec<&str> = input
			.split(|c| c == '[' || c == '-' || c == ' ' || c == ']' || c == ':')
			.filter(|token| !token.is_empty())
			.collect();
		if tokens.len() < 7 {
			return err!("Invalid input line \"{}\"", input)
		}

		let month: u32 = tokens[1].parse()?;
		let mut day: u32 = tokens[2].parse()?;

		let hour = tokens[3];
		let mut minute = tokens[4].parse()?;
		if hour == "23" {
			minute -= 60;
			day += 1;
		}

		let event_token = tokens[5];
		let event = match event_token {
			"Guard" => {
				let guard_token = tokens[6];
				let id = guard_token[1..].parse()?;
				Ok(InputEvent::BeginShift(id))
			},
			"falls" => Ok(InputEvent::FallAsleep),
			"wakes" => Ok(InputEvent::Wake),
			_ => err!("Invalid event \"{}\"", tokens[5]),
		}?;

		Ok(Input {
			day: day + (month * 31),
			minute,
			event,
		})
	}
}

#[derive(Debug)]
struct GuardRecord {
	guard: u32, // guard on duty
	day: u32, // day of the year
	sleep_periods: Vec<(i32, i32)>, // (fell asleep, woke up]
}

impl GuardRecord {
	fn total_minutes_slept(&self) -> u32{
		return self.sleep_periods
			.iter()
			.fold(0, |val, el| val + (el.1 - el.0)) as u32;
	}

	fn sleep_periods_from(slice: &[Input]) -> Result<Vec<(i32, i32)>> {
		let mut result = Vec::new();
		let mut last_day = None;
		let mut iter = slice.iter();
		while let Some(sleep) = iter.next() {
			if last_day.is_some() && last_day.unwrap() != sleep.day {
				return err!("iter contains events from different days!")
			}
			last_day = Some(sleep.day);

			let wake = iter.next();
			if wake.is_none() {
				return err!("missing wake event");
			}
			let wake = wake.unwrap();

			let valid_events =
				sleep.event == InputEvent::FallAsleep &&
				wake.event == InputEvent::Wake;

			if valid_events {
				result.push((sleep.minute, wake.minute))
			} else {
				return err!("unexpected event types {:?}, {:?}", sleep.event, wake.event);
			}
		}
		return Ok(result);
	}

	fn from_records(input: &[Input]) -> Result<Vec<GuardRecord>> {
		let mut result = Vec::new();

		let mut idx = 0;
		loop {
			if idx >= input.len() {
				return Ok(result);
			}

			let shift_start = &input[idx];
			let guard = match shift_start.event {
				InputEvent::BeginShift(id) => Ok(id),
				_ => err!("input @ {} not shift start", idx),
			}?;

			let mut next_idx = None;
			for night in input[idx + 1..].iter().enumerate() {
				match night.1.event {
					InputEvent::BeginShift(_) => {
						next_idx = Some(night.0);
						break;
					},
					_ => continue,
				}
			}

			let next_idx = if let Some(i) = next_idx {
				idx + 1 + i
			} else {
				input.len()
			};

			let slice = &input[idx + 1..next_idx];
			result.push(GuardRecord {
				guard,
				day: shift_start.day,
				sleep_periods: GuardRecord::sleep_periods_from(slice)?,
			});

			idx = next_idx;
		}
	}
}

pub fn p1(input: &str) -> Result<u32> {
	let mut lines: Vec<&str> = input.lines().collect();
	lines.sort();

	let events: Result<Vec<_>> = lines.iter().map(|line| Input::from_str(line)).collect();

	let guard_records = GuardRecord::from_records(&events?)?;

	let mut guards: HashMap<u32, Vec<GuardRecord>> = HashMap::new();
	for record in guard_records {
		let entry = guards.entry(record.guard).or_default();
		entry.push(record);
	}

	let mut chosen_guard = (0, 0);
	for (guard, records) in guards.iter() {
		let mins_slept = records.iter().fold(
			0,
			|total, record| total + record.total_minutes_slept());
		if mins_slept > chosen_guard.1 {
			chosen_guard = (*guard, mins_slept);
		}
	}

	let mut minutes = [0; 60];
	let guard_records = &guards[&chosen_guard.0];
	for record in guard_records.iter() {
		for sleep_period in record.sleep_periods.iter() {
			let range = sleep_period.0..sleep_period.1;
			for min in range {
				minutes[min as usize] += 1;
			}
		}
	}

	let most_slept_minute = minutes.iter().enumerate().max_by(|x, y| x.1.cmp(y.1));
	if most_slept_minute.is_none() {
		return err!("Could not find minute spent asleep the most");
	}
	let most_slept_minute = most_slept_minute.unwrap();

	return Ok(chosen_guard.0 * most_slept_minute.0 as u32);
}

pub fn p2(input: &str) -> Result<u32> {
	let mut lines: Vec<&str> = input.lines().collect();
	lines.sort();

	let events: Result<Vec<_>> = lines.iter().map(|line| Input::from_str(line)).collect();

	let guard_records = GuardRecord::from_records(&events?)?;

	let mut guards: HashMap<u32, Vec<GuardRecord>> = HashMap::new();
	for record in guard_records {
		let entry = guards.entry(record.guard).or_default();
		entry.push(record);
	}

	let mut result = (0, (0, 0));
	let mut minutes = [0; 60];
	for (guard, records) in guards.iter() {
		// reset array
		for min in &mut minutes[0..] { *min = 0; }

		for record in records.iter() {
			for sleep_period in record.sleep_periods.iter() {
				let range = sleep_period.0..sleep_period.1;
				for min in range {
					minutes[min as usize] += 1;
				}
			}
		}

		let max = minutes.iter().enumerate().max_by(|x, y| x.1.cmp(y.1));
		if max.is_none() {
			return err!("failed to find max?");
		}
		let max = max.unwrap();
		if *max.1 > (result.1).1 {
			result = (*guard, (max.0, *max.1));
		}
	}

	return Ok(result.0 * (result.1).0 as u32);
}
