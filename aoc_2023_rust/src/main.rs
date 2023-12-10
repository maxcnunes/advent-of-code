use std::env;

mod day01;

fn main() {
    println!("Advent of Code 2023");

    let task = env::args()
        .nth(1)
        .expect("missing task name (e.g. day01_part01)");

    match task.as_str() {
        "day01_part01" => day01::part01::process(),
        "day01_part02" => day01::part02::process(),
        _ => panic!("Invalid task {task}"),
    };
}
