pub fn part_one() -> String {
    let mut input = vec![
        vec!["S", "Z", "P", "D", "L", "B", "F", "C"],
        vec!["N","V", "G", "P", "H", "W", "B"], 
        vec!["F", "W", "B", "J", "G"],
        vec!["G", "J", "N", "F", "L", "W", "C", "S"], 
        vec!["W", "J", "L", "T", "P", "M", "S", "H"], 
        vec!["B", "C", "W", "G", "F", "S"], 
        vec!["H", "T", "P", "P", "M", "Q", "B", "W"],
        vec!["F", "S", "W", "T"], 
        vec!["N", "C", "R"],
    ];
    let input_str = include_str!(r"../../Inputs/Day5/input.txt");
    let (_, instructions) = input_str.split_once("\n\n").unwrap();
    for line in instructions.lines() {
        println!("{line}");
        let (num_moved, from, to) = match &line.split_ascii_whitespace().skip(1).step_by(2).take(3).collect::<Vec<_>>()[..] {
            &[a, b, c, ..] => (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap(), c.parse::<usize>().unwrap()),
            _ => unreachable!()
        };
        for _ in 0..num_moved {
            let test = input[from-1].pop().unwrap();
            input[to-1].push(test);
        }
    }
    let mut solution = String::new();
    for stack in 0..input.len() {
      solution.push_str(input[stack].pop().unwrap()); 
    }
    solution
    
    
}

pub fn part_two() -> String {
    let mut input = vec![
        vec!["S", "Z", "P", "D", "L", "B", "F", "C"],
        vec!["N","V", "G", "P", "H", "W", "B"], 
        vec!["F", "W", "B", "J", "G"],
        vec!["G", "J", "N", "F", "L", "W", "C", "S"], 
        vec!["W", "J", "L", "T", "P", "M", "S", "H"], 
        vec!["B", "C", "W", "G", "F", "S"], 
        vec!["H", "T", "P", "P", "M", "Q", "B", "W"],
        vec!["F", "S", "W", "T"], 
        vec!["N", "C", "R"],
    ];
    let input_str = include_str!(r"../../Inputs/Day5/input.txt");
    let (_, instructions) = input_str.split_once("\n\n").unwrap();
    for line in instructions.lines() {
        println!("{line}");
        let (num_moved, from, to) = match &line.split_ascii_whitespace().skip(1).step_by(2).take(3).collect::<Vec<_>>()[..] {
            &[a, b, c, ..] => (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap(), c.parse::<usize>().unwrap()),
            _ => unreachable!()
        };
        let splitting_index = input[from-1].len() - num_moved;
        let mut split_vec = input[from-1].split_off(splitting_index);
        input[to-1].append(&mut split_vec);

    }
    let mut solution = String::new();
    for stack in 0..input.len() {
      solution.push_str(input[stack].pop().unwrap()); 
    }
    solution
    
}
