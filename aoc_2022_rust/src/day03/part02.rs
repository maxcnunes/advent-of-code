use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

// Find the item type that corresponds to the badges of each three-Elf group.
// What is the sum of the priorities of those item types?
pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let total = cal_priorities(lines)?;

    println!("{} Total number of priorities: {}", icon::CHECK_MARK, total);

    Ok(())
}

fn cal_priorities(lines: io::Lines<io::BufReader<File>>) -> Result<usize, SolutionError> {
    let mut total: usize = 0;
    let mut i: usize = 0;
    let mut group_lines: Vec<String> = vec![];

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;

        group_lines.push(line);

        if (i + 1) % 3 == 0 {
            total += get_backpack_priority(&group_lines)?;
            group_lines.clear();
        }

        i += 1;
    }

    Ok(total)
}

fn get_backpack_priority(lines: &Vec<String>) -> Result<usize, SolutionError> {
    let alphabet: Vec<_> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let items_2: HashMap<char, bool> = lines[1].chars().map(|c| (c, true)).collect();
    let items_3: HashMap<char, bool> = lines[2].chars().map(|c| (c, true)).collect();

    for char_1 in lines[0].chars() {
        let found = match (items_2.get(&char_1), items_3.get(&char_1)) {
            (Some(true), Some(true)) => true,
            _ => false,
        };

        if !found {
            // Not found for this char, move to the next one
            continue;
        }

        let char_low = char_1.to_lowercase().to_string();
        let alphatbet_index = alphabet
            .iter()
            .position(|c| c.to_string() == char_low)
            .ok_or(SolutionError::CharNotFoundErr(char_1))?;

        let result = if char_1.is_lowercase() {
            alphatbet_index + 1
        } else {
            alphatbet_index + 27
        };

        // println!("Found match char {char_1} alphatbet_index {alphatbet_index} result {result}",);
        return Ok(result);
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
        // - 18 (r) for the first group
        // - 52 (Z) for the second group.
        // The sum of these is 70.
        let lines = input::load(3, true)?;
        let total = cal_priorities(lines)?;
        assert_eq!(total, 70);
        Ok(())
    }

    #[test]
    fn real_result_ok() -> Result<(), SolutionError> {
        let lines = input::load(3, false)?;
        let total = cal_priorities(lines)?;
        assert_eq!(total, 2805);
        Ok(())
    }
}
