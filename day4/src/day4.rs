use core::num;

use regex::Regex;

pub fn part_one() -> i32 {
    let input = include_str!(r"../../Inputs/Day4/input.txt");
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut num_pairs = 0;
    for line in input.lines() {
        let capture = re.captures_iter(line).nth(0).unwrap();
        let first_lower:i32 = capture[1].parse::<i32>().unwrap();
        let first_higher = capture[2].parse::<i32>().unwrap();
        let second_lower = capture[3].parse::<i32>().unwrap();
        let second_higher = capture[4].parse::<i32>().unwrap();
        if (first_lower <= second_lower && first_higher >= second_higher) || (second_lower <= first_lower && second_higher >= first_higher) {
            num_pairs += 1;
        }
    }
    num_pairs
}


pub fn part_two() -> i32 {
    let input = include_str!(r"../../Inputs/Day4/input.txt");
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut num_pairs = 0;
    for line in input.lines() {
        let capture = re.captures_iter(line).nth(0).unwrap();
        let first_lower:i32 = capture[1].parse::<i32>().unwrap();
        let first_higher = capture[2].parse::<i32>().unwrap();
        let second_lower = capture[3].parse::<i32>().unwrap();
        let second_higher = capture[4].parse::<i32>().unwrap();

        if (first_lower <= second_higher && second_lower <= first_higher) || (second_lower <= first_higher && first_lower <= second_higher) {
            num_pairs += 1;
        }
        
    }
    num_pairs
}