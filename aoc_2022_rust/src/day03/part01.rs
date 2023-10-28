use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

// Find the item type that appears in both compartments of each rucksack.
// What is the sum of the priorities of those item types?
pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let total = calc_priorities(lines)?;

    println!("{} Total number of priorities: {}", icon::CHECK_MARK, total);

    Ok(())
}

fn calc_priorities(lines: io::Lines<io::BufReader<File>>) -> Result<usize, SolutionError> {
    let mut total: usize = 0;

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;

        total += get_backpack_priority(line)?;
    }

    Ok(total)
}

fn get_backpack_priority(line: String) -> Result<usize, SolutionError> {
    let alphabet: Vec<_> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let items: Vec<_> = line.chars().collect();

    let size = items.len();
    let half = size / 2;

    for i_f in 1..=half {
        let f = items[i_f - 1];
        let f_low = f.to_lowercase().to_string();
        let alphatbet_index = alphabet
            .iter()
            .position(|c| c.to_string() == f_low)
            .ok_or(SolutionError::CharNotFoundErr(f))?;

        for i_s in half + 1..=size {
            let s = items[i_s - 1];
            if f == s {
                let result = if f.is_lowercase() {
                    alphatbet_index + 1
                } else {
                    alphatbet_index + 27
                };

                // println!("Found match char {f} alphatbet_index {alphatbet_index} result {result}",);
                return Ok(result);
            }
        }
    }

    Err(SolutionError::CouldNotFindPriorityErr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SolutionError;
    use crate::input;

    #[test]
    fn demo_result_ok() -> Result<(), SolutionError> {
        // Expected results:
        // 1. 16 (p)
        // 2. 38 (L)
        // 3. 42 (P)
        // 4. 22 (v)
        // 5. 20 (t)
        // 6. 19 (s)
        // the sum of these is 157.
        let lines = input::load(3, true)?;
        let total = calc_priorities(lines)?;
        assert_eq!(total, 157);
        Ok(())
    }

    #[test]
    fn real_result_ok() -> Result<(), SolutionError> {
        let lines = input::load(3, false)?;
        let total = calc_priorities(lines)?;
        assert_eq!(total, 7674);
        Ok(())
    }
}
