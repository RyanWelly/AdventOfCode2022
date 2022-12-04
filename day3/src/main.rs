use std::collections::{HashMap, HashSet};

fn main() {
    println!("Answer for part one: {}", part_one());
    println!("Answer for part two: {}", part_two());
}

pub fn part_one() -> i32 {
    let mut priority_sum = 0;
    let input = include_str!(r"../../Inputs/Day3/input.txt");
    let alphabet_range: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    for line in input.lines() {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let hashset: HashSet<char> = first_half.chars().collect();
        priority_sum += second_half
            .chars()
            .filter(|c| hashset.contains(c))
            .collect::<HashSet<char>>()
            .iter()
            .map(|c| alphabet_range.get(&c).unwrap() + 1)
            .sum::<usize>();
    }
    priority_sum as i32
}

pub fn part_two() -> i32 {
    let input = include_str!(r"../../Inputs/Day3/input.txt");
    let alphabet_range: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(num, a)| (num + 1, a))
        .map(|(a, b)| (b, a))
        .collect();

    input
        .lines()
        .step_by(3)
        .zip(input.lines().skip(1).step_by(3))
        .zip(input.lines().skip(2).step_by(3))
        .map(|((a, b), c)| {
            println!("{a}, {b}, {c}");
            a.chars()
                .filter(|char| b.contains(*char) && c.contains(*char))
                .next()
                .unwrap()
        })
        .map(|char| *alphabet_range.get(&char).unwrap())
        .sum::<usize>() as i32
}
