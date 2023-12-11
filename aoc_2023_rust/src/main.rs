use std::env;

mod day01;

fn main() {
    println!("");
    println!(":::::::::::::::::::::::::::");
    println!("::: Advent of Code 2023 :::");
    println!(":::::::::::::::::::::::::::");
    println!("");

    let task = env::args()
        .nth(1)
        .expect("Missing task name (e.g. day01_part01)");

    let result = match task.as_str() {
        "day01_part01" => day01::part01::process(),
        "day01_part02" => day01::part02::process(),
        _ => panic!("Invalid task {task}"),
    };

    println!(" => Result {task}: {result}");
}
