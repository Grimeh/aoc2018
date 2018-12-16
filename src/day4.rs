use std::error::Error;
use std::collections::HashMap;

struct GuardRecord {
	day: u16, // day of the year
	time: i8, // minutes from midnight
}

//impl GuardRecord {
//	fn total_time_slept(&self) -> Result<u32> {
//		let mut result: u32 = 0;
//		for period in self.sleep_periods.iter() {
//			let diff = period.1 - period.0;
//			if diff < 0 {
//				return err!("Invalid period (negative time): {}-{}", period.0, period.1);
//			}
//			result += diff as u32;
//		}
//		return result;
//	}
//}

fn d4p1(input: &str) -> Result<u32> {
	let mut records = HashMap::new();

	let mut lines: Vec<&str> = input.lines().collect();
	lines.sort();
	for line in lines.iter() {

	}
}