use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn score(&self) -> i32 {
        match *self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        }
    }
}

enum RoundResult {
    Lose,
    Draw,
    Win
}

impl RoundResult {
    fn score(&self) -> i32 {
        match *self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6
        }
    }

    fn from_round(round: &Round) -> Self {
        use Choice::*;
        use RoundResult::*;
        
        match (round.0, round.1) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Lose,

            (Paper, Rock) => Lose,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Win,

            (Scissors, Rock) => Win,
            (Scissors, Paper) => Lose,
            (Scissors, Scissors) => Draw
        }
    }
}

struct Round(Choice, Choice);

impl Round {
    pub fn score(&self) -> i32 {
        RoundResult::from_round(self).score() + self.1.score()
    }

    fn from_desired_result_as_string(line: &String) -> Self {
        use Choice::*;

        match line.as_str() {
            "A X" => Round(Rock, Scissors),
            "A Y" => Round(Rock, Rock),
            "A Z" => Round(Rock, Paper),
            "B X" => Round(Paper, Rock),
            "B Y" => Round(Paper, Paper),
            "B Z" => Round(Paper, Scissors),
            "C X" => Round(Scissors, Paper),
            "C Y" => Round(Scissors, Scissors),
            "C Z" => Round(Scissors, Rock),
            _ => panic!("error parsing line containing string '{}'", line)
        }
    }

    fn from_round_as_string(line: &String) -> Self{
        use Choice::*;

        match line.as_str() {
            "A X" => Round(Rock, Rock),
            "A Y" => Round(Rock, Paper),
            "A Z" => Round(Rock, Scissors),
            "B X" => Round(Paper, Rock),
            "B Y" => Round(Paper, Paper),
            "B Z" => Round(Paper, Scissors),
            "C X" => Round(Scissors, Rock),
            "C Y" => Round(Scissors, Paper),
            "C Z" => Round(Scissors, Scissors),
            _ => panic!("error parsing line containing string '{}'", line)
        }

    }
}


fn main() {
    let input_file = File::open("../resources/input.txt").or_else(|_| File::open("./resources/input.txt"));
    let lines = input_file
    .map(|input| io::BufReader::new(input).lines());

    let lines: Vec<String> = match lines {
        Ok(lines_buffer) => lines_buffer.into_iter().map(|line| line.expect("error parsing input line")).collect(),
        Err(error) => panic!("input error: {}", error.to_string())
    };

    let result_part_1: i32 = lines.iter().map(Round::from_round_as_string).map(|round| round.score()).sum();
    let result_part_2: i32 = lines.iter().map(Round::from_desired_result_as_string).map(|round| round.score()).sum();

    println!("result part 1: {}", result_part_1);
    println!("result part 2: {}", result_part_2);
}
