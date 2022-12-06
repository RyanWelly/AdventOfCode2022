fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}


pub fn part_one() -> i32 {
    let input = include_str!(r"../../Inputs/Day6/input.txt");
    input.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .take_while(|&a| {
            for i in 1..a.len() {
                if a[i..].contains(&a[i - 1]) {
                    return true;
                }
            }
            false
        }).count() as i32 + 4i32
}


pub fn part_two() -> i32 {
    let input = include_str!(r"../../Inputs/Day6/input.txt");
    input.chars()
        .collect::<Vec<char>>()
        .windows(14)
        .take_while(|&a| {
            for i in 1..a.len() {
                if a[i..].contains(&a[i - 1]) {
                    return true;
                }
            }
            false
        }).count() as i32 + 14i32

}
