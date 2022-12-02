use std::{collections::HashMap, fs::read_to_string};

//A and X are rock
//B and Y are paper
//C and Z are scissors

fn main() {
    println!("Part one solution: {}", part_one());
    println!("Part two solution: {}", part_two());

}

pub fn part_one() -> i32 {
    let mut score = 0;
    let input_file = read_to_string("../Inputs/Day2/input.txt").unwrap();
    for entry in input_file.split("\n") {
        let (opp_move, my_move) = entry.split_at(1);
        //points for which option is played
        match my_move {
            " X" => score += 1,
            " Y" => score += 2,
            " Z" => score += 3,
            _ => panic!("Invalid move ({my_move})"),
        };
        //points for whether you've won (6), lost (0) or drawn(3)
        match (opp_move, my_move) {
            ("A", " Y") => score += 6,
            ("A", " X") => score += 3,
            ("A", " Z") => score += 0,

            ("B", " Z") => score += 6,
            ("B", " Y") => score += 3,
            ("B", " X") => score += 0,

            ("C", " X") => score += 6,
            ("C", " Z") => score += 3,
            ("C", " Y") => score += 0,

            _ => panic!("failed at {} {}", opp_move, my_move),
        }
    }
    score
}

pub fn part_two() -> i32 {
    let mut score = 0;
    let input_file = read_to_string("../Inputs/Day2/input.txt").unwrap();

    let win_bonus = 6;
    let draw_bonus = 3;
    let lose_bonus = 0;    



    let rock = 0;
    let paper = 1;
    let scissors = 2;
    let symbol_to_move = {let mut hash = HashMap::new();
    hash.insert("A", rock);
    hash.insert("B", paper);
    hash.insert("C", scissors);
    hash
    };

    let win = ["Paper", "Scissors", "Rock"];
    let draw = ["Rock", "Paper", "Scissors"];
    let lose = ["Scissors", "Rock", "Paper"];

    let move_to_value = {let mut map = HashMap::new();
    map.insert("Rock", 1);
    map.insert("Paper",2);
    map.insert("Scissors",3);
    map
    };
    for entry in input_file.split("\n") {
        let (opp_move, result) = entry.split_at(1);
        score += match (opp_move, result) {
            (played, " Z") => move_to_value.get(win[*symbol_to_move.get(played).unwrap()]).unwrap() + win_bonus ,
            (played, " Y") => move_to_value.get(draw[*symbol_to_move.get(played).unwrap()]).unwrap() + draw_bonus,
            (played, " X") => move_to_value.get(lose[*symbol_to_move.get(played).unwrap()]).unwrap() + lose_bonus,
            (_, _) => panic!("invalid second value of {result}"),
        }
    }

    score
}
    
