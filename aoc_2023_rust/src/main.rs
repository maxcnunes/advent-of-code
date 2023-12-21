use std::env;

mod day01;
mod day02;
mod day03;

fn main() {
    println!("");
    println!(":::::::::::::::::::::::::::");
    println!("::: Advent of Code 2023 :::");
    println!(":::::::::::::::::::::::::::");
    println!("");

    let puzzle = env::args()
        .nth(1)
        .expect("Missing puzzle name (e.g. day01_part01)");

    let result = match puzzle.as_str() {
        "day01_part01" => day01::part01::process(),
        "day01_part02" => day01::part02::process(),
        "day02_part01" => day02::part01::process(),
        "day02_part02" => day02::part02::process(),
        "day03_part01" => day03::part01::process(),
        "day03_part02" => day03::part02::process(),
        _ => panic!("Invalid puzzle {puzzle}"),
    };

    println!(" => Result {puzzle}: {result}");
}
