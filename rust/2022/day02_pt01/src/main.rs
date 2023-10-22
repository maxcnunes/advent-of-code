use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = match env::var("DEMO") {
        Ok(_) => "demo-input.txt",
        Err(_) => "input.txt",
    };

    println!("Using input data {}", file_name);

    let lines = read_lines(file_name).unwrap();

    let mut first_user_score: u32 = 0;
    let mut second_user_score: u32 = 0;

    for line_result in lines {
        let line = line_result.unwrap();

        let round: Vec<&str> = line.split(' ').collect();

        let options = parse_round(&round);

        // (0 if you lost, 3 if the round was a draw, and 6 if you won)
        const LOST_VALUE: u32 = 0;
        const DRAW_VALUE: u32 = 3;
        const WON_VALUE: u32 = 6;

        match options {
            // Handle draws
            (GameOption::Rock, GameOption::Rock)
            | (GameOption::Paper, GameOption::Paper)
            | (GameOption::Scissors, GameOption::Scissors) => {
                first_user_score += DRAW_VALUE;
                second_user_score += DRAW_VALUE;
                // println!("DRAW");
            }
            // Handle first users won
            (GameOption::Rock, GameOption::Scissors)
            | (GameOption::Paper, GameOption::Rock)
            | (GameOption::Scissors, GameOption::Paper) => {
                // println!("LOST"); // second user
                first_user_score += WON_VALUE;
                second_user_score += LOST_VALUE;
            }
            // Handle second user won
            (_, _) => {
                // println!("WON"); // second user
                first_user_score += LOST_VALUE;
                second_user_score += WON_VALUE;
            }
        }

        let (first, second) = options;
        // println!("first={:?} second={:?}", first, second);

        let first_value = get_option_value(&first);
        let second_value = get_option_value(&second);

        first_user_score += first_value;
        second_user_score += second_value;
    }

    println!("Final result:");
    println!("  - first user got {} points", first_user_score);
    println!("  - second user got {} points", second_user_score);
}

#[derive(Debug)]
enum GameOption {
    Rock,
    Paper,
    Scissors,
}

// TODO: convert to Result
fn parse_round(round: &Vec<&str>) -> (GameOption, GameOption) {
    // First column: A for Rock, B for Paper, and C for Scissors.
    let first = match round[0] {
        "A" => GameOption::Rock,
        "B" => GameOption::Paper,
        "C" => GameOption::Scissors,
        _ => panic!("Invalid option"),
    };

    // Second column: X for Rock, Y for Paper, and Z for Scissors.
    let second = match round[1] {
        "X" => GameOption::Rock,
        "Y" => GameOption::Paper,
        "Z" => GameOption::Scissors,
        _ => panic!("Invalid option"),
    };

    (first, second)
}

fn get_option_value(option: &GameOption) -> u32 {
    match option {
        GameOption::Rock => 1,
        GameOption::Paper => 2,
        GameOption::Scissors => 3,
    }
}

// Uses a buffer and iterator to read a file instead of loading the whole
// file in memory at once.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
