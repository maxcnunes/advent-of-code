use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let total_top_calories = calc_elves_top_calories(lines)?;

    println!(
        "{} The sum of the calories carried by top three elves is {}.",
        icon::CHECK_MARK,
        total_top_calories,
    );

    Ok(())
}

fn calc_elves_top_calories(lines: io::Lines<io::BufReader<File>>) -> Result<u32, SolutionError> {
    let mut current_elf: u8 = 1;
    let mut current_total_calories: u32 = 0;
    let mut elves_calories: Vec<(u8, u32)> = vec![];

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;

        if line == "" {
            elves_calories.push((current_elf, current_total_calories));

            current_elf += 1;
            current_total_calories = 0;
            continue;
        }

        let calories: u32 = line.parse().map_err(SolutionError::ParseLineErr)?;
        current_total_calories += calories;
    }

    // record last elf calories
    elves_calories.push((current_elf, current_total_calories));

    // reverse sorting by calories (most calories at the top)
    elves_calories.sort_by(|a, b| b.1.cmp(&a.1));

    let top_3_elves: Vec<_> = elves_calories.iter().take(3).collect();
    // println!("  Top 3 elves {:?}", top_3_elves);

    let total_top_calories: u32 = top_3_elves
        .into_iter()
        .map(|elf| elf.1) // get callories
        .sum();

    Ok(total_top_calories)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SolutionError;
    use crate::input;

    #[test]
    fn demo_result_ok() -> Result<(), SolutionError> {
        // Expected results:
        // The top three Elves are the fourth Elf (with 24000 Calories),
        // then the third Elf (with 11000 Calories),
        // then the fifth Elf (with 10000 Calories).
        // The sum of the Calories carried by these three elves is 45000.
        let lines = input::load(1, true)?;
        let total = calc_elves_top_calories(lines)?;
        assert_eq!(total, 45000);
        Ok(())
    }

    #[test]
    fn real_result_ok() -> Result<(), SolutionError> {
        let lines = input::load(1, false)?;
        let total = calc_elves_top_calories(lines)?;
        assert_eq!(total, 200158);
        Ok(())
    }
}
