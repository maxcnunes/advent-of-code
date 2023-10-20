use std::env;
use std::fs::read_to_string;

fn main() {
    let file_name = match env::var("DEMO") {
        Ok(_) => "demo-input.txt",
        Err(_) => "input.txt",
    };

    println!("Using input data {}", file_name);

    let lines = read_lines(file_name);

    let mut elf_most_calories = 0;
    let mut total_most_calories = 0;

    let mut current_elf = 1;
    let mut current_total_calories = 0;

    for line in lines {
        if line == "" {
            current_elf += 1;
            current_total_calories = 0;
            continue;
        }

        let calories: i32 = line.parse().unwrap();
        current_total_calories += calories;

        if current_total_calories > total_most_calories {
            elf_most_calories = current_elf;
            total_most_calories = current_total_calories;
        }
    }
    println!(
        "The elf {} is carrying the most calories {}.",
        elf_most_calories, total_most_calories
    );
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
