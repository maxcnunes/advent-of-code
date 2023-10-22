use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process(demo: bool) {
    let file_name = match demo {
        true => "src/day01/demo-input.txt",
        false => "src/day01/input.txt",
    };

    let lines = read_lines(file_name).unwrap();

    let mut elf_most_calories: u8 = 0;
    let mut total_most_calories: u32 = 0;

    let mut current_elf: u8 = 1;
    let mut current_total_calories: u32 = 0;

    for line_result in lines {
        let line = line_result.unwrap();

        if line == "" {
            current_elf += 1;
            current_total_calories = 0;
            continue;
        }

        let calories: u32 = line.parse().unwrap();
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

// Uses a buffer and iterator to read a file instead of loading the whole
// file in memory at once.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
