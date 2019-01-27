#![feature(test)]
#![allow(dead_code)]

extern crate test;
extern crate hashbrown;

#[macro_use]
mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

#[cfg(test)]
mod tests {
    use test::Bencher;

    fn file_to_str(filename: &str) -> String {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(filename).expect("file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("could not read file contents");

        return contents;
    }

    #[test]
    fn day1p1() {
    	let contents = file_to_str("day1.txt");
    	let value = ::day1::d1p1(&contents);
    	println!("Day 1 part 1 result: {}", value.unwrap());
    }

    #[test]
    fn day1p2() {
    	let contents = file_to_str("day1.txt");
    	let value = ::day1::d1p2(&contents);
    	println!("Day 1 part 2 result: {}", value.unwrap());
    }

    #[test]
    fn day2p1() {
        let contents = file_to_str("day2.txt");
        let value = ::day2::p1(&contents);
        println!("Day 2 part 1 result: {}", value);
    }

    #[test]
    fn day2p2() {
        let contents = file_to_str("day2.txt");
        let value = ::day2::p2(&contents).expect("failed to find the boxes");
        println!("Day 2 part 2 result: {}", value);
    }

    #[test]
    fn day3p1() {
        let contents = file_to_str("day3.txt");
        let value = ::day3::p1(&contents).unwrap();
        println!("Day 3 part 1 result: {}", value);
    }

    #[bench]
    fn day3p1_vector(bencher: &mut Bencher) {
        bencher.iter(|| {
            let contents = file_to_str("day3.txt");
            let value = ::day3::p1(&contents).unwrap();
            println!("Day 3 part 1 result: {}", value);
        });
    }

    #[bench]
    fn day3p1_hashmap(bencher: &mut Bencher) {
        bencher.iter(|| {
            let contents = file_to_str("day3.txt");
            let value = ::day3::p1_hashmap(&contents).unwrap();
            println!("Day 3 part 1 result: {}", value);
        });
    }
    
    #[test]
    fn day3p2() {
        let contents = file_to_str("day3.txt");
        let value = ::day3::p2(&contents).unwrap();
        println!("Day 3 part 2 result: {}", value);
    }

    #[bench]
    fn day3p2_vector(bencher: &mut Bencher) {
        bencher.iter(|| {
            let contents = file_to_str("day3.txt");
            let value = ::day3::p2(&contents).unwrap();
            println!("Day 3 part 2 result: {}", value);
        });
    }

    #[bench]
    fn day3p2_hashmap(bencher: &mut Bencher) {
        bencher.iter(|| {
            let contents = file_to_str("day3.txt");
            let value = ::day3::p2_hashmap(&contents).unwrap();
            println!("Day 3 part 2 result: {}", value);
        });
    }

    #[test]
    fn day4p1() {
        let contents = file_to_str("day4.txt");
        let value = ::day4::p1(&contents).unwrap();
        println!("Day 4 part 1 result: {}", value);
    }

    #[test]
    fn day4p2() {
        let contents = file_to_str("day4.txt");
        let value = ::day4::p2(&contents).unwrap();
        println!("Day 4 part 2 result: {}", value);
    }

    #[test]
    fn day5p1() {
        let contents = file_to_str("day5.txt");
        let value = ::day5::p1(&contents);
        println!("Day 5 part 1 result: {}", value);
    }

    #[test]
    fn day5p2() {
        let contents = file_to_str("day5.txt");
        let value = ::day5::p2(&contents);
        println!("Day 5 part 2 result: {}", value);
    }
}
