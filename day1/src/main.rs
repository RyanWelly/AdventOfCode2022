use std::collections::BinaryHeap;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    println!("{}", part_one());
    println!("{}", part_two());
}

//shamelessly copied from Rust by Example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part_one() -> i32 {
    if let Ok(lines) = read_lines("../Inputs/Day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_max = 0;
        let mut current_carry = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    current_max = std::cmp::max(current_max, current_carry);
                    current_carry = 0;
                    continue;
                }
                let num = ip.parse::<i32>().unwrap();
                current_carry += num;
            }
        }
        return current_max;
    }
    0
}

pub fn part_two() -> i32 {
    if let Ok(lines) = read_lines("../Inputs/Day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_carry = 0;
        let mut top_three = BinaryHeap::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    top_three.push(current_carry);
                    current_carry = 0;
                    continue;
                }
                let num = ip.parse::<i32>().unwrap();
                current_carry += num;
            }
        }
        return top_three.into_sorted_vec().iter().rev().take(3).sum();
    }
    0
}
