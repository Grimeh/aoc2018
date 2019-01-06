// macro stolen from https://github.com/BurntSushi/advent-of-code/blob/master/aoc03/src/main.rs
// Disclaimer: I have no clue how rust macros work just yet
#[macro_use]
macro_rules! err {
	($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

use std::error::Error;
pub type Result<T> = std::result::Result<T, Box<Error>>;
