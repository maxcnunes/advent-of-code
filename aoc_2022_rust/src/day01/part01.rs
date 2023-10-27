use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;

pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let mut elf_most_calories: u8 = 0;
    let mut total_most_calories: u32 = 0;

    let mut current_elf: u8 = 1;
    let mut current_total_calories: u32 = 0;

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;

        if line == "" {
            current_elf += 1;
            current_total_calories = 0;
            continue;
        }

        let calories: u32 = line.parse().map_err(SolutionError::ParseLineErr)?;
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

    Ok(())
}
