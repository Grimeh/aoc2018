mod day1;
mod day2;
mod day3;

#[cfg(test)]
mod tests {
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
    	println!("Day 1 part 1 result: {}", value);
    }

    #[test]
    fn day1p2() {
    	let contents = file_to_str("day1.txt");
    	let value = ::day1::d1p2(&contents);
    	println!("Day 1 part 2 result: {}", value);
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
        let value = ::day2::p2(&contents);
        println!("Day 2 part 2 result: {}", value);
    }

    #[test]
    fn day3p1() {
        let contents = file_to_str("day3.txt");
        let value = ::day3::p1(&contents);
        println!("Day 3 part 1 result: {}", value);
    }
}
