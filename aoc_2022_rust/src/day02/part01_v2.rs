use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process(demo: bool) {
    let file_name = match demo {
        true => "src/day02/demo-input.txt",
        false => "src/day02/input.txt",
    };

    let lines = read_lines(file_name).unwrap();

    let mut first_user_score: u32 = 0;
    let mut second_user_score: u32 = 0;

    for line_result in lines {
        let line = line_result.unwrap();

        let round: Vec<&str> = line.split(' ').collect();

        let (first, second) = parse_round(&round);

        // (0 if you lost, 3 if the round was a draw, and 6 if you won)
        const LOST_VALUE: u32 = 0;
        const DRAW_VALUE: u32 = 3;
        const WON_VALUE: u32 = 6;

        if first.choice == second.choice {
            first_user_score += DRAW_VALUE;
            second_user_score += DRAW_VALUE;
        } else if first.beats == second.choice {
            // println!("LOST"); // second user
            first_user_score += WON_VALUE;
            second_user_score += LOST_VALUE;
        } else {
            // println!("WON"); // second user
            first_user_score += LOST_VALUE;
            second_user_score += WON_VALUE;
        }

        // println!("first={:?} second={:?}", first, second);

        let first_value = first.points;
        let second_value = second.points;

        first_user_score += first_value;
        second_user_score += second_value;
    }

    println!("Final result:");
    println!("  - first user got {} points", first_user_score);
    println!("  - second user got {} points", second_user_score);
}

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
struct Play {
    choice: Choice,
    beats: Choice,
    points: u32,
}

impl Play {
    fn new(choice: Choice) -> Play {
        match choice {
            Choice::Rock => Play {
                choice: Choice::Rock,
                beats: Choice::Scissors,
                points: 1,
            },
            Choice::Paper => Play {
                choice: Choice::Paper,
                beats: Choice::Rock,
                points: 2,
            },
            Choice::Scissors => Play {
                choice: Choice::Scissors,
                beats: Choice::Paper,
                points: 3,
            },
        }
    }
}

// TODO: convert to Result
fn parse_round(round: &Vec<&str>) -> (Play, Play) {
    // First column: A for Rock, B for Paper, and C for Scissors.
    let first = match round[0] {
        "A" => Play::new(Choice::Rock),
        "B" => Play::new(Choice::Paper),
        "C" => Play::new(Choice::Scissors),
        _ => panic!("Invalid option"),
    };

    // Second column: X for Rock, Y for Paper, and Z for Scissors.
    let second = match round[1] {
        "X" => Play::new(Choice::Rock),
        "Y" => Play::new(Choice::Paper),
        "Z" => Play::new(Choice::Scissors),
        _ => panic!("Invalid option"),
    };

    (first, second)
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
