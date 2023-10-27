use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let mut first_user_score: u32 = 0;
    let mut second_user_score: u32 = 0;

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;

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

    println!("{} Final result:", icon::CHECK_MARK);
    println!("  - first user got {} points", first_user_score);
    println!("  - second user got {} points", second_user_score);

    Ok(())
}

#[derive(Debug)]
enum GameOption {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Result {
    Lost,
    Draw,
    Won,
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

    // Second column:
    //  X means you need to lose,
    //  Y means you need to end the round in a draw,
    //  and Z means you need to win
    let second_expected_result = match round[1] {
        "X" => Result::Lost,
        "Y" => Result::Draw,
        "Z" => Result::Won,
        _ => panic!("Invalid option"),
    };

    let second = match (&first, second_expected_result) {
        (GameOption::Rock, Result::Lost) => GameOption::Scissors,
        (GameOption::Rock, Result::Draw) => GameOption::Rock,
        (GameOption::Rock, Result::Won) => GameOption::Paper,

        (GameOption::Paper, Result::Lost) => GameOption::Rock,
        (GameOption::Paper, Result::Draw) => GameOption::Paper,
        (GameOption::Paper, Result::Won) => GameOption::Scissors,

        (GameOption::Scissors, Result::Lost) => GameOption::Paper,
        (GameOption::Scissors, Result::Draw) => GameOption::Scissors,
        (GameOption::Scissors, Result::Won) => GameOption::Rock,
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
